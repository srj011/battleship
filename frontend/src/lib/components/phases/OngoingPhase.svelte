<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import Fleet from '$lib/components/Fleet.svelte';
    import type { CellView, Coord, FireMessage, PreviewCell } from '$lib/types';

    const playerFleet = $derived($gameStore.game?.player_fleet);
    const opponentFleet = $derived($gameStore.game?.opponent_fleet);

    const isMyTurn = $derived(
        $gameStore.game?.status.type === 'ongoing' &&
            $gameStore.game.player !== null &&
            $gameStore.game.player === $gameStore.game.turn
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

        <div class="flex gap-16">
            <!-- Player's section -->
            <div class="flex flex-col rounded p-2">
                <!-- Player's Board -->
                <Board
                    class={`${isMyTurn ? 'opacity-60' : ''}`}
                    board={$gameStore.game.player_board}
                />
                <h2 class="my-3 text-center text-sm">Your board</h2>

                <!-- Player's fleet -->
                {#if playerFleet}
                    <Fleet fleet={playerFleet} variant="player" />
                {/if}
            </div>

            <!-- Opponent's section-->
            <div class="flex flex-col rounded p-2">
                <!-- Opponent's Board -->
                <Board
                    class={`${!isMyTurn ? 'opacity-60' : ''}`}
                    board={$gameStore.game.opponent_board}
                    clickable={isMyTurn}
                    onCellClick={handleFire}
                    {isCellClickable}
                />
                <h2 class="my-3 text-center text-sm">Opponent's board</h2>

                <!-- Opponent's fleet -->
                {#if opponentFleet}
                    <Fleet fleet={opponentFleet} variant="opponent" />
                {/if}
            </div>
        </div>
    {/if}
</div>
