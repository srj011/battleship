<script lang="ts">
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
        Direction
    } from '$lib/types';
    import { isWithinBounds } from '$lib/game/utils';
    import Icon from '@iconify/svelte';

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

<div class="flex h-full w-full">
    <!-- Board controls -->
    <div class="flex flex-1 items-start justify-center">
        <div class="flex scale-100 flex-col items-center gap-8 pt-8 2xl:scale-105">
            <div class="flex flex-col items-center gap-2">
                <h1 class="text-2xl font-semibold tracking-wide uppercase">PREVIEW BOARD</h1>
                <p class="text-xs tracking-widest uppercase">PLACE SHIPS TO THE BOARD TO BEGIN</p>
            </div>

            <!-- Board Preview -->
            <Board
                board={previewBoard}
                clickable={!$gameStore.game?.player_ready}
                onRightClick={handleRightClick}
                onPointerEnter={handleHover}
                onPointerUp={handleDrop}
                onPointerDown={handleDrag}
                onPointerLeave={handleLeave}
            />

            <!-- Controls-->
            <div class="flex gap-4">
                <button
                    class="flex items-center gap-3 rounded-md border px-4 py-2 disabled:opacity-50"
                    onclick={placeFleet}
                    disabled={placements.length !== TOTAL_SHIPS || $gameStore.game?.player_ready}
                >
                    <Icon icon="material-symbols:check-circle-rounded" width="20" height="20" />
                    {isWaiting ? 'Placement Confirmed' : 'Confirm Placement'}
                </button>
                <button
                    class="flex items-center gap-3 rounded border px-4 py-2"
                    onclick={handleReset}
                >
                    <Icon icon="ri:reset-left-fill" width="20" height="20" />
                    Reset Board
                </button>
                <button
                    class="flex items-center gap-3 rounded border px-4 py-2"
                    onclick={generateRandomFleet}
                >
                    <Icon icon="material-symbols:shuffle-rounded" width="20" height="20" />
                    Randomize
                </button>
            </div>
        </div>
    </div>

    <!-- Fleet panel -->
    <div class="flex h-full w-90 shrink-0 flex-col gap-6 border border-neutral-500/80 p-6">
        <h1 class="text-xl font-semibold tracking-tight uppercase">FLEET DEPLOYMENT</h1>
        <div class="grid grid-cols-1 gap-4">
            {#each ships as ship (ship)}
                {@const isDragging = dragState?.ship_type === ship}

                <button
                    class={`flex flex-col gap-4 border border-neutral-600/50 px-4 py-4 select-none
                    ${isPlaced(ship) || isDragging ? 'opacity-40' : 'cursor-grab hover:bg-gray-50'}`}
                    onpointerdown={() => {
                        if (isPlaced(ship)) return;
                        startDrag(ship);
                    }}
                >
                    <div class="flex justify-between text-xs">
                        <span class="font-semibold tracking-wider uppercase">{ship}</span>
                        <span class="text-[0.7rem] tracking-tighter opacity-50"
                            >SIZE: {SHIP_LENGTHS[ship]}</span
                        >
                    </div>

                    <!-- Ship bar -->
                    <div class="flex gap-2">
                        {#each Array(SHIP_LENGTHS[ship]) as _, i (i)}
                            <div class="h-4 w-15 border-2 border-indigo-500 bg-indigo-500/80"></div>
                        {/each}
                    </div>
                </button>
            {/each}
        </div>
        <!-- Progress bar -->
        <div class="mt-auto flex flex-col gap-1">
            <div class="flex items-center justify-between">
                <h3 class="text-xs tracking-wide uppercase">DEPLOYMENT PROGRESS</h3>
                <h2 class="text-sm font-semibold">{progress}%</h2>
            </div>
            <div class="h-1 w-full bg-gray-300">
                <div class="h-1 bg-indigo-500 transition-all" style={`width: ${progress}%`}></div>
            </div>
        </div>
    </div>
</div>
