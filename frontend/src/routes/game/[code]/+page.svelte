<script lang="ts">
    import PlacingShipsPhase from '$lib/components/phases/PlacingShipsPhase.svelte';
    import OngoingPhase from '$lib/components/phases/OngoingPhase.svelte';
    import FinishedPhase from '$lib/components/phases/FinishedPhase.svelte';

    import { onMount, onDestroy } from 'svelte';
    import { gameStore } from '$lib/stores/game';
    import { page } from '$app/state';
    import { connectWS, disconnectWS } from '$lib/api/websocket';

    const code: string = $derived(page.params.code ?? '');
    const token = $derived(page.url.searchParams.get('player_token') ?? '');

    onMount(() => {
        connectWS(code, token);
    });

    onDestroy(() => {
        disconnectWS();
    });
</script>

<div class="flex justify-center gap-6 p-4">
    {#if !$gameStore.connected}
        <div class="flex flex-1 items-center justify-center gap-3 text-black">
            <div class="h-2 w-2 animate-pulse rounded-full bg-black"></div>
            <p>Connecting to game...</p>
        </div>
    {:else if !$gameStore.game}
        <p>Loading game...</p>
    {:else if $gameStore.game.status.type === 'placing_ships'}
        <PlacingShipsPhase />
    {:else if $gameStore.game.status.type === 'ongoing'}
        <OngoingPhase />
    {:else if $gameStore.game.status.type === 'finished'}
        <FinishedPhase />
    {/if}
</div>
