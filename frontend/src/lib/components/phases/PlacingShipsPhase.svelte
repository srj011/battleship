<script lang="ts">
    import { onMount } from 'svelte';
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import { BOARD_SIZE, SHIP_LENGTHS, TOTAL_SHIPS } from '$lib/game/config';
    import type {
        ClientMessage,
        ShipPlacement,
        ShipType,
        Coord,
        PreviewBoard,
        PreviewCell,
        Direction,
        CellView
    } from '$lib/types';
    import { isWithinBounds } from '$lib/game/utils';

    const ships: ShipType[] = ['carrier', 'battleship', 'destroyer', 'submarine', 'patrolboat'];

    const isWaiting = $derived($gameStore.game?.player_ready && !$gameStore.game.opponent_ready);

    const placementsMap = $state<Record<ShipType, ShipPlacement | null>>({
        carrier: null,
        battleship: null,
        destroyer: null,
        submarine: null,
        patrolboat: null
    });

    const placements = $derived(
        Object.values(placementsMap).filter((p): p is ShipPlacement => p !== null)
    );
    const progress = $derived((placements.length / ships.length) * 100);

    let isInsideBoard = $state(false);

    const isPlaced = (ship: ShipType) => placementsMap[ship] !== null;

    type DragState = {
        ship_type: ShipType;
        anchor: Coord | null;
        direction: Direction;
        previous: ShipPlacement | null;
    } | null;

    let dragState = $state<DragState>(null);

    const previewBoard: PreviewBoard = $derived.by(() => {
        // Create empty grid
        const { cells } = createEmptyBoard();

        // Place existing ships
        for (const ship of placements) {
            for (const cell of getShipCells(ship.ship_type, ship.start, ship.direction)) {
                cells[cell.row][cell.col] = { type: 'placed', ship_type: ship.ship_type };

                // Mark surrounding cells as blocked
                for (let dr = -1; dr <= 1; dr++) {
                    for (let dc = -1; dc <= 1; dc++) {
                        const row = cell.row + dr;
                        const col = cell.col + dc;

                        if (!isWithinBounds({ row, col })) {
                            continue;
                        }

                        if (cells[row][col].type === 'placed') continue;
                        if (cells[row][col].type === 'blocked') continue;

                        cells[row][col] = { type: 'blocked' };
                    }
                }
            }
        }

        // Add preview
        if (dragState && dragState.anchor) {
            const clampedStart = clampStart(
                dragState.ship_type,
                dragState.anchor,
                dragState.direction
            );

            const shipCells = getShipCells(dragState.ship_type, clampedStart, dragState.direction);
            const valid = isValidPlacement(shipCells);

            for (const cell of shipCells) {
                cells[cell.row][cell.col] = {
                    type: valid ? 'preview-valid' : 'preview-invalid',
                    ship_type: dragState.ship_type
                };
            }
        }
        return { cells };
    });

    onMount(() => {
        generateRandomFleet();
    });

    $effect(() => {
        if (!$gameStore.randomFleet) return;

        for (const placement of $gameStore.randomFleet) {
            placementsMap[placement.ship_type] = placement;
        }
        dragState = null;
    });

    function startDrag(ship_type: ShipType) {
        dragState = {
            ship_type,
            anchor: null,
            direction: 'horizontal',
            previous: placementsMap[ship_type]
        };

        placementsMap[ship_type] = null;
    }

    function handleDrag(coord: Coord) {
        if (dragState) return;

        const existing = getShipAt(coord);
        if (!existing) return;

        dragState = {
            ship_type: existing.ship_type,
            anchor: coord,
            direction: existing.direction,
            previous: existing
        };

        placementsMap[existing.ship_type] = null;
    }

    function handleHover(coord: Coord) {
        if (!dragState) return;

        isInsideBoard = true;
        dragState.anchor = coord;
    }

    function handleDrop() {
        if (!dragState) return;

        if (!isInsideBoard || !dragState.anchor) {
            const ship = dragState.ship_type;
            placementsMap[ship] = null;
            dragState = null;
            return;
        }

        const clampedStart = clampStart(dragState.ship_type, dragState.anchor, dragState.direction);

        const cells: Coord[] = getShipCells(dragState.ship_type, clampedStart, dragState.direction);

        if (!isValidPlacement(cells)) {
            if (dragState.previous) {
                placementsMap[dragState.ship_type] = dragState.previous;
            }
            dragState = null;
            return;
        }

        placementsMap[dragState.ship_type] = {
            ship_type: dragState.ship_type,
            start: clampedStart,
            direction: dragState.direction
        };
        dragState = null;
    }

    function handleRightClick(coord: Coord) {
        if (dragState) {
            toggleDirection();
            return;
        }

        const existing = getShipAt(coord);
        if (!existing) return;

        dragState = {
            ship_type: existing.ship_type,
            anchor: existing.start,
            direction: existing.direction === 'horizontal' ? 'vertical' : 'horizontal',
            previous: existing
        };

        placementsMap[existing.ship_type] = null;
    }

    function handleCancel() {
        if (!dragState) return;

        if (dragState.previous) {
            placementsMap[dragState.ship_type] = dragState.previous;
        }
        dragState = null;
    }

    function handleLeave() {
        if (!dragState) return;

        isInsideBoard = false;
        dragState.anchor = null;
    }

    function handleReset() {
        for (const ship of ships) {
            placementsMap[ship] = null;
        }
    }

    function createEmptyBoard(): PreviewBoard {
        return {
            cells: Array.from({ length: BOARD_SIZE }, () =>
                Array.from({ length: BOARD_SIZE }, () => ({ type: 'empty' }))
            )
        };
    }

    function toggleDirection() {
        if (!dragState) return;
        dragState.direction = dragState.direction === 'horizontal' ? 'vertical' : 'horizontal';
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
        for (const cell of cells) {
            // Bounds check
            if (!isWithinBounds(cell)) return false;

            // Overlap and blocked check
            for (const ship of placements) {
                const existing = getShipCells(ship.ship_type, ship.start, ship.direction);
                for (const e of existing) {
                    const dr = Math.abs(cell.row - e.row);
                    const dc = Math.abs(cell.col - e.col);

                    if (dr <= 1 && dc <= 1) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    function clampStart(ship: ShipType, start: Coord, direction: Direction): Coord {
        const length = SHIP_LENGTHS[ship];

        if (direction === 'horizontal') {
            return {
                row: start.row,
                col: Math.min(start.col, BOARD_SIZE - length)
            };
        } else {
            return {
                row: Math.min(start.row, BOARD_SIZE - length),
                col: start.col
            };
        }
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
</script>

<svelte:window
    onkeydown={(e) => {
        if (e.key === 'Escape') handleCancel();
        if (e.key.toLowerCase() === 'r') toggleDirection();
    }}
    onpointerup={handleDrop}
/>

<div class="flex flex-col items-center gap-6">
    <h2 class="text-xl font-semibold">Place your fleet</h2>

    <!-- Board Preview -->
    <div>
        <h3 class="mb-2 text-lg font-medium">Preview Board</h3>
        <Board
            board={previewBoard}
            clickable={!$gameStore.game?.player_ready}
            onRightClick={handleRightClick}
            onPointerEnter={handleHover}
            onPointerUp={handleDrop}
            onPointerDown={handleDrag}
            onPointerLeave={handleLeave}
        />
    </div>

    <!-- Controls-->
    <div class="flex gap-4">
        <button class="rounded bg-blue-500 px-4 py-2 text-white" onclick={generateRandomFleet}>
            Randomize Fleet
        </button>
        <button
            class="rounded bg-green-500 px-4 py-2 text-white disabled:opacity-50"
            onclick={placeFleet}
            disabled={placements.length !== TOTAL_SHIPS || $gameStore.game?.player_ready}
        >
            {isWaiting ? 'Waiting for Opponent...' : 'Confirm Placement'}
        </button>
    </div>

    <div>
        {#if isWaiting}
            <p>Waiting for opponent...</p>
        {/if}
        {#if $gameStore.game?.opponent_ready}
            <p>Opponent ready</p>
        {/if}
    </div>
</div>
