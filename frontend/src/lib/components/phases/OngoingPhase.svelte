<script lang="ts">
	import { sendWS } from '$lib/api/websocket';
	import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import type { Coord, FireMessage } from '$lib/types';

    function handleFire(args: {coord: Coord}) {
        let msg: FireMessage = {
            type: "fire",
            coord: args.coord,
        };
        sendWS(msg);
    }
</script>

<div class="gap-6">
    <h1 class="text-2xl font-bold">Game</h1>

    {#if !$gameStore.connected}
        <p class="text-red-400">Connecting...</p>
    {/if}

    {#if $gameStore.game}
        <div class="flex gap-10">

            <p>Turn: {$gameStore.game.turn}</p>

            <!-- Player's board -->
            <div>
                <h2 class="text-lg font-semibold mb-2">Your Board</h2>
                <Board board={$gameStore.game.player_board} />
            </div>

            <!-- Opponent's board-->
            <div>
                <h2 class="text-lg font-semibold mb-2">Opponent's Board</h2>
                <Board
                    board={$gameStore.game.opponent_board}
                    clickable={true}
                    onCellClick={handleFire}
                />
            </div>
        </div>
    {/if}
</div>
