<script lang="ts">
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
</script>

<div class="flex flex-col items-center gap-2">
    {#if $gameStore.game}
        <!-- GameOver Status -->
        {#if $gameStore.game.status.type === 'finished'}
            <div
                class={`${
                    $gameStore.game.status.winner === $gameStore.player
                        ? 'bg-green-600'
                        : 'bg-red-600'
                }
                    w-full max-w-3xl rounded px-4 py-3 text-white`}
            >
                <div class="flex items-center justify-between">
                    <span class="font-semibold">
                        {$gameStore.game.status.winner === $gameStore.player
                            ? 'Game over. You won!'
                            : 'Game over. You lost.'}
                    </span>
                    <button class="ml-4 rounded bg-gray-200 px-2 py-1 text-sm text-black">
                        Play Again
                    </button>
                </div>
            </div>
        {/if}
        <!-- Board's flexbox -->
        <div class="flex gap-16">
            <!-- Player's board -->
            <div class="rounded p-2">
                <Board board={$gameStore.game.player_board} />
                <p class="my-3 text-center text-sm">Your board</p>
            </div>

            <!-- Opponent's board-->
            <div class="rounded p-2">
                <Board board={$gameStore.game.opponent_board} />
                <p class="my-3 text-center text-sm">Opponent's board</p>
            </div>
        </div>
    {/if}
</div>
