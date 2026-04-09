<script lang="ts">
    import { gameStore } from '$lib/stores/game';
    import { sendWS } from '$lib/api/websocket';
    import Board from '$lib/components/Board.svelte';
    import Fleet from '$lib/components/Fleet.svelte';
    import type { ClientMessage } from '$lib/types';

    const playerFleet = $derived($gameStore.game?.player_fleet);
    const opponentFleet = $derived($gameStore.game?.opponent_fleet);

    const rematchSelf = $derived($gameStore.game?.player_rematch_ready ?? false);
    const rematchOpponent = $derived($gameStore.game?.opponent_rematch_ready ?? false);

    function handleRestart() {
        const msg: ClientMessage = { type: 'restart' };
        sendWS(msg);
    }
</script>

<div class="flex flex-col items-center gap-2">
    {#if $gameStore.game}
        <!-- GameOver Status -->
        {#if $gameStore.game.status.type === 'finished'}
            <div
                class={`${
                    $gameStore.game.status.winner === $gameStore.game.player
                        ? 'bg-green-600'
                        : 'bg-red-600'
                }
                    w-full max-w-3xl rounded px-4 py-3 text-white`}
            >
                <div class="flex items-center justify-between">
                    <span class="font-semibold">
                        {$gameStore.game.status.winner === $gameStore.game.player
                            ? 'Game over. You won!'
                            : 'Game over. You lost.'}
                    </span>
                </div>
                <button
                    class="cursor-pointer rounded-sm bg-gray-900/90 px-4
                    py-2 text-sm font-semibold text-white uppercase
                    hover:bg-gray-800/80"
                    onclick={handleRestart}
                    disabled={!rematchSelf}
                >
                    {#if rematchSelf}
                        {#if rematchOpponent}
                            Starting new game...
                        {:else}
                            Waiting for opponent...
                        {/if}
                    {:else if rematchOpponent}
                        Accept Rematch
                    {:else}
                        Play Again
                    {/if}
                </button>
            </div>
        {/if}

        <div class="flex gap-16">
            <!-- Player's board -->
            <div class="rounded p-2">
                <Board board={$gameStore.game.player_board} />
                <p class="my-3 text-center text-sm">Your board</p>

                <!-- Player's fleet -->
                {#if playerFleet}
                    <Fleet fleet={playerFleet} variant="player" />
                {/if}
            </div>

            <!-- Opponent's board-->
            <div class="rounded p-2">
                <Board board={$gameStore.game.opponent_board} />
                <p class="my-3 text-center text-sm">Opponent's board</p>

                <!-- Opponent's fleet -->
                {#if opponentFleet}
                    <Fleet fleet={opponentFleet} variant="opponent" />
                {/if}
            </div>
        </div>
    {/if}
</div>
