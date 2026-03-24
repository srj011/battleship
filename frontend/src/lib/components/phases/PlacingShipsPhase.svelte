<script lang="ts">
    import { sendWS } from "$lib/api/websocket";
    import { gameStore } from "$lib/stores/game";
    import Board from "$lib/components/Board.svelte";
	import type { 
        ClientMessage,
        ShipPlacement,
        BoardView, 
		CellView

    } from "$lib/types";

    const EMPTY_CELL: CellView = { type: 'empty'};

    function createEmptyBoard(): BoardView {
        return {
            cells: Array.from({ length: 10}, () => Array.from({ length: 10 }, () => EMPTY_CELL))
        };
    }

    function applyPreviewFleet(fleet: ShipPlacement[] | null) {
        const board = createEmptyBoard();

        if (!fleet) return board;

        const lengthMap = {
            carrier: 5,
            battleship: 4,
            destroyer: 3,
            submarine: 3,
            patrolboat: 2
        };

        for (const ship of fleet) {
            const { ship_type, start, direction } = ship;
            const length = lengthMap[ship_type];

            for (let i=0; i < length; i++) {
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

    const previewBoard = $derived(applyPreviewFleet($gameStore.previewFleet));

    function generateRandomFleet() {
        let msg: ClientMessage = { type: 'random_fleet'};
        sendWS(msg);
    }

    function placeFleet() {
        if (!$gameStore.previewFleet) return;

        let msg: ClientMessage = {
            type: 'place_fleet',
            fleet: $gameStore.previewFleet
        };
        sendWS(msg);
    }
</script>

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
