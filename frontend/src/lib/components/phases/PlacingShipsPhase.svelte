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

    const isWaiting = $derived($gameStore.game?.player_ready && !$gameStore.game.opponent_ready);

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
        if (activeShip && hoverCoord) {
            const clampedStart = clampStart(activeShip.ship_type, hoverCoord, activeShip.direction);

            const shipCells = getShipCells(
                activeShip.ship_type,
                clampedStart,
                activeShip.direction
            );
            const valid = isValidPlacement(shipCells);

            for (const cell of shipCells) {
                cells[cell.row][cell.col] = {
                    type: valid ? 'preview-valid' : 'preview-invalid',
                    ship_type: activeShip.ship_type
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
        activeShip.direction = activeShip.direction === 'horizontal' ? 'vertical' : 'horizontal';
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

    function handleHover(coord: Coord) {
        hoverCoord = coord;
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

        const clampedStart = clampStart(activeShip.ship_type, coord, activeShip.direction);
        const cells = getShipCells(activeShip.ship_type, clampedStart, activeShip.direction);
        if (!isValidPlacement(cells)) return;

        placements.push({
            ...activeShip,
            start: clampedStart
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
            activeShip = rotated;
            committedShip = existing;
        }
    }

    function handleCancel() {
        if (!activeShip || !committedShip) return;

        placements.push(committedShip);
        activeShip = null;
        committedShip = null;
    }

    function isCellClickable(cell: PreviewCell | CellView): boolean {
        if (activeShip) {
            return cell.type === 'empty' || cell.type === 'preview-valid';
        } else {
            return cell.type === 'placed';
        }
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
        <h3 class="mb-2 text-lg font-medium">Preview Board</h3>
        <Board
            board={previewBoard}
            clickable={!$gameStore.game?.player_ready}
            onCellClick={handleClick}
            onRightClick={handleRightClick}
            onCellHover={handleHover}
            {isCellClickable}
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
