<script lang="ts">
    import PlacingShipsPhase from '$lib/components/phases/PlacingShipsPhase.svelte';
    import OngoingPhase from '$lib/components/phases/OngoingPhase.svelte';
    import FinishedPhase from '$lib/components/phases/FinishedPhase.svelte';
    import DisconnectOverlay from '$lib/components/DisconnectOverlay.svelte';

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

    const TIMEOUT_MS = 30_000;

    let seconds = $state(0);
    let interval: ReturnType<typeof setInterval> | null = null;

    $effect(() => {
        const disconnect = $gameStore.playerDisconnect;

        if (disconnect && disconnect.player !== $gameStore.game?.player) {
            const update = () => {
                const now = Date.now();
                const elapsed = now - disconnect.disconnected_at;
                const remaining = Math.max(0, TIMEOUT_MS - elapsed);
                seconds = Math.ceil(remaining / 1000);

                if (remaining <= 0 && interval) {
                    clearInterval(interval);
                    interval = null;
                }
            };
            update();

            interval = setInterval(update, 1000);
        } else {
            if (interval) {
                clearInterval(interval);
                interval = null;
            }
            seconds = 0;
        }
    });

    const phase = $derived($gameStore.game?.status.type);
</script>

<div class="flex flex-1 justify-center gap-6 p-4">
    {#if !$gameStore.connected}
        <div class="flex flex-1 items-center justify-center gap-3 text-black">
            <div class="h-2 w-2 animate-pulse rounded-full bg-black"></div>
            <p>Connecting to game...</p>
        </div>
    {:else if !$gameStore.game}
        <p>Loading game...</p>
    {:else if phase === 'placing_ships'}
        <PlacingShipsPhase />
    {:else if phase === 'ongoing'}
        <OngoingPhase />
    {:else}
        <FinishedPhase />
    {/if}

    {#if $gameStore.playerDisconnect && $gameStore.playerDisconnect.player !== $gameStore.game?.player && seconds > 0}
        <DisconnectOverlay {seconds} />
    {/if}
</div>
