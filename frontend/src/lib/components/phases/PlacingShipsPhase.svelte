<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import { BOARD_SIZE, SHIP_LENGTHS, TOTAL_SHIPS } from '$lib/game/config';
    import type {
        ClientMessage,
        ShipPlacement,
        BoardView,
        ShipType,
        Coord,
        PreviewBoard,
        PreviewCell,
        Direction
    } from '$lib/types';
    import { onMount } from 'svelte';

    let activeShip = $state<ShipPlacement | null>(null);
    let committedShip = $state<ShipPlacement | null>(null);
    let placements = $state<ShipPlacement[]>([]);
    let hoverCoord = $state<Coord | null>(null);

    const previewBoard: PreviewBoard = $derived.by(() => {
        // Create empty grid
        const { cells } = createEmptyBoard();

        // Place existing ships
        for (const ship of placements) {
            for (const cell of getShipCells(ship.ship_type, ship.start, ship.direction)) {
                cells[cell.row][cell.col] = { type: 'placed', ship_type: ship.ship_type };
            }
        }

        // Add preview
        if (activeShip && hoverCoord) {
            const shipCells = getShipCells(activeShip.ship_type, hoverCoord, activeShip.direction);
            const valid = isValidPlacement(shipCells);

            for (const cell of shipCells) {
                if (
                    cell.row < 0 ||
                    cell.col < 0 ||
                    cell.row >= BOARD_SIZE ||
                    cell.col >= BOARD_SIZE
                )
                    continue;

                cells[cell.row][cell.col] = {
                    type: valid ? 'preview-valid' : 'preview-invalid',
                    ship_type: activeShip.ship_type
                };
            }
        }
        return { cells };
    });

    onMount(() => {
        const msg: ClientMessage = { type: 'random_fleet' };
        sendWS(msg);
    });

    $effect(() => {
        if (!$gameStore.randomFleet) return;

        placements = $gameStore.randomFleet;
        activeShip = null;
        committedShip = null;
    });

    function createEmptyBoard(): PreviewBoard {
        return {
            cells: Array.from({ length: BOARD_SIZE }, () =>
                Array.from({ length: BOARD_SIZE }, () => ({ type: 'empty' }))
            )
        };
    }

    function toggleDirection() {
        if (!activeShip) return;
        activeShip.direction = activeShip?.direction === 'horizontal' ? 'vertical' : 'horizontal';
    }

    function getShipAt(coord: Coord): ShipPlacement | null {
        const cell = previewBoard.cells[coord.row][coord.col];
        if (cell.type !== 'placed') return null;

        return placements.find((p) => p.ship_type === cell.ship_type) ?? null;
    }

    function getShipCells(ship: ShipType, start: Coord, direction: Direction): Coord[] {
        const length = SHIP_LENGTHS[ship];
        const cells: Coord[] = [];

        for (let i = 0; i < length; i++) {
            const row = direction === 'vertical' ? start.row + i : start.row;
            const col = direction === 'horizontal' ? start.col + i : start.col;

            cells.push({ row, col });
        }
        return cells;
    }

    function isValidPlacement(cells: Coord[]): boolean {
        // Bounds check
        for (const cell of cells) {
            if (cell.row < 0 || cell.col < 0 || cell.row >= BOARD_SIZE || cell.col >= BOARD_SIZE) {
                return false;
            }
        }

        // Overlap check
        for (const ship of placements) {
            const existing = getShipCells(ship.ship_type, ship.start, ship.direction);
            for (const cell of cells) {
                if (existing.some((e) => e.row == cell.row && e.col == cell.col)) {
                    return false;
                }
            }
        }
        return true;
    }

    function applyPreviewFleet(fleet: ShipPlacement[] | null) {
        const board = createEmptyBoard();

        if (!fleet) return board;

        for (const ship of fleet) {
            const { ship_type, start, direction } = ship;
            const length = SHIP_LENGTHS[ship_type];

            for (let i = 0; i < length; i++) {
                const row = direction === 'vertical' ? start.row + i : start.row;
                const col = direction === 'horizontal' ? start.col + i : start.col;

                board.cells[row][col] = {
                    type: 'ship',
                    ship_type
                };
            }

        }
        return board;
    }

    function generateRandomFleet() {
        const msg: ClientMessage = { type: 'random_fleet' };
        sendWS(msg);
    }

    function placeFleet() {
        if (placements.length !== TOTAL_SHIPS) return;

        const msg: ClientMessage = {
            type: 'place_fleet',
            fleet: placements
        };
        sendWS(msg);
    }

    function handleClick(coord: Coord) {
        const existing = getShipAt(coord);

        if (existing) {
            if (activeShip && committedShip) {
                placements.push(committedShip);
            }

            placements = placements.filter((p) => p !== existing);
            activeShip = { ...existing };
            committedShip = existing;
            return;
        }

        if (!activeShip) return;

        const cells = getShipCells(activeShip.ship_type, coord, activeShip.direction);
        if (!isValidPlacement(cells)) return;

        placements.push({
            ...activeShip,
            start: coord
        });

        activeShip = null;
        committedShip = null;
    }

    function handleRightClick(coord: Coord) {
        // Active ship
        if (activeShip) {
            toggleDirection();
            return;
        }

        //Rotate ship in place
        const existing = getShipAt(coord);
        if (!existing) return true;

        placements = placements.filter((p) => p !== existing);
        const rotated: ShipPlacement = {
            ...existing,
            direction: existing.direction === 'horizontal' ? 'vertical' : 'horizontal'
        };
        const cells = getShipCells(rotated.ship_type, rotated.start, rotated.direction);
        if (isValidPlacement(cells)) {
            placements.push(rotated);
        } else {
            placements.push(existing);
        }
    }

    function handleCancel() {
        if (!activeShip || !committedShip) return;

        placements.push(committedShip);
        activeShip = null;
        committedShip = null;
    }

</script>

<svelte:window
    onkeydown={(e) => {
        if (e.key === 'Escape') handleCancel();
        if (e.key.toLowerCase() === 'r') toggleDirection();
    }}
/>

<div class="flex flex-col items-center gap-6">
    <h2 class="text-xl font-semibold">Place your fleet</h2>

    <!-- Board Preview -->
    <div>
        <h3 class="text-lg font-medium mb-2">Preview Board</h3>
        <Board board={previewBoard} />
    </div>

    <!-- Controls-->
    <div class="flex gap-4">
        <button 
            class="px-4 py-2 bg-blue-500 text-white rounded"
            onclick={generateRandomFleet}
        >
            Randomize Fleet
        </button>
        <button
            class="px-4 py-2 bg-green-500 text-white rounded disabled:opacity-50"
            onclick={placeFleet}
            disabled={!$gameStore.previewFleet}
        >
            Confirm Placement
        </button>
    </div> 
</div>
