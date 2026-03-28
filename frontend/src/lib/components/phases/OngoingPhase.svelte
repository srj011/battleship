<script lang="ts">
	import { sendWS } from '$lib/api/websocket';
	import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import type { CellView, Coord, FireMessage, PreviewCell } from '$lib/types';

    const isMyTurn = $derived(
        $gameStore.game?.status.type === "ongoing" &&
        $gameStore.player !== null &&
        $gameStore.player === $gameStore.game.turn
    );

    function handleFire(coord: Coord) {
        if (!isMyTurn) return;

        const msg: FireMessage = {
            type: 'fire',
            coord
        };
        sendWS(msg);
    }

    function isCellClickable(cell: CellView | PreviewCell): boolean {
        return cell.type === 'empty' || cell.type === 'unknown';
    }
</script>

<div class="flex flex-col items-center gap-2">
    <h1 class="text-2xl font-bold">Game</h1>

    {#if !$gameStore.connected}
        <p class="text-red-400">Connecting...</p>
    {/if}

    {#if $gameStore.game}
        <!-- Turn Indicator -->
        <div class={`text-lg font-semibold ${
            isMyTurn ? "text-green-500" : "text-gray-400"
        }`}>
            {isMyTurn ? "Your Turn" : "Opponent's Turn"}
        </div>

        <!-- Board's flexbox -->
        <div class="flex gap-16">

            <!-- Player's board -->
            <div class={`p-2 rounded ${
                isMyTurn ? "opacity-60" : ""
            }`}>
                <Board board={$gameStore.game.player_board} />
                <p class="text-sm text-center my-3">Your board</p>
            </div>

            <!-- Opponent's board-->
            <div class={`p-2 rounded ${
                !isMyTurn ? "opacity-60" : ""
            }`}>
                <Board
                    board={$gameStore.game.opponent_board}
                    clickable={isMyTurn}
                    onCellClick={handleFire}
                    {isCellClickable}
                />
                <p class="text-sm text-center my-3">Opponent's board</p>
            </div>
        </div>
    {/if}
</div>
