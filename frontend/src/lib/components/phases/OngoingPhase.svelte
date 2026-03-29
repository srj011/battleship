<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import type { CellView, Coord, FireMessage, PreviewCell } from '$lib/types';

    const isMyTurn = $derived(
        $gameStore.game?.status.type === 'ongoing' &&
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
    {#if $gameStore.game}
        <!-- Turn Indicator -->
        <div class="w-full max-w-3xl rounded bg-gray-200 px-4 py-3 text-sm text-black">
            <div class="flex items-center justify-center">
                <span class="font-semibold">
                    {isMyTurn ? 'Your Turn' : "Opponent's Turn"}
                </span>
            </div>
        </div>

        <!-- Board's flexbox -->
        <div class="flex gap-16">
            <!-- Player's board -->
            <div class={`rounded p-2 ${isMyTurn ? 'opacity-60' : ''}`}>
                <Board board={$gameStore.game.player_board} />
                <p class="my-3 text-center text-sm">Your board</p>
            </div>

            <!-- Opponent's board-->
            <div class={`rounded p-2 ${!isMyTurn ? 'opacity-60' : ''}`}>
                <Board
                    board={$gameStore.game.opponent_board}
                    clickable={isMyTurn}
                    onCellClick={handleFire}
                    {isCellClickable}
                />
                <p class="my-3 text-center text-sm">Opponent's board</p>
            </div>
        </div>
    {/if}
</div>
