<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { page } from '$app/state';
	import { connectWS, disconnectWS, sendWS } from '$lib/api/websocket';
	import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import type { Coord, FireMessage, GameStatus } from '$lib/types';

	const code: string = $derived(page.params.code ?? '');
	const token = $derived(page.url.searchParams.get('player_token') ?? '');

    onMount(() => {
        connectWS(code, token);
    });

    onDestroy(() => {
        disconnectWS();
    });

    function handleFire(args: {coord: Coord}) {
        let msg: FireMessage = {
            type: "fire",
            coord: args.coord,
        };
        sendWS(msg);
    }

    function getStatusText(status: GameStatus) {
        switch (status.type) {
            case "placing_ships":
                return "Placing Ships";
            case "ongoing":
                return "Ongoing";
            case "finished":
                return `Winner: ${status.winner}`;
  }
}
</script>

<div class="min-h-screen p-4 flex flex-col items-center justify-center gap-6">
    <h1 class="text-2xl font-bold">Game {code}</h1>

    {#if !$gameStore.connected}
        <p class="text-red-400">Connecting...</p>
    {/if}

    {#if $gameStore.game}
        <div class="flex gap-10">

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

        <div>
            <p>{getStatusText($gameStore.game.status)}</p>
            
            {#if $gameStore.game.status.type === 'ongoing'}
                <p>Turn: {$gameStore.game.turn}</p>
            {/if}
            
            
        </div>
    {/if}
</div>

