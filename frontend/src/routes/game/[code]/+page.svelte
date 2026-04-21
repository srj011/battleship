<script lang="ts">
    import PlacingShipsPhase from '$lib/components/phases/PlacingShipsPhase.svelte';
    import OngoingPhase from '$lib/components/phases/OngoingPhase.svelte';
    import FinishedPhase from '$lib/components/phases/FinishedPhase.svelte';
    import DisconnectOverlay from '$lib/components/DisconnectOverlay.svelte';

    import { onMount, onDestroy } from 'svelte';
    import { beforeNavigate } from '$app/navigation';
    import { gameStore } from '$lib/stores/game';
    import { page } from '$app/state';
    import { connectWS, disconnectWS } from '$lib/api/websocket';

    const code: string = $derived(page.params.code ?? '');
    const token = $derived(page.url.searchParams.get('player_token') ?? '');

    onMount(() => {
        gameStore.reset();
        connectWS(code, token);

        const handler = (e: BeforeUnloadEvent) => {
            if (
                $gameStore.game?.status.type === 'placing_ships' ||
                $gameStore.game?.status.type === 'ongoing'
            ) {
                e.preventDefault();
            }
        };
        window.addEventListener('beforeunload', handler);

        return () => {
            window.removeEventListener('beforeunload', handler);
        };
    });

    beforeNavigate((nav) => {
        if (
            $gameStore.game?.status.type === 'placing_ships' ||
            $gameStore.game?.status.type === 'ongoing'
        ) {
            if (!confirm('Leave game? Match will be forfeit.')) {
                nav.cancel();
            }
        }
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
    {#if !$gameStore.game}
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
