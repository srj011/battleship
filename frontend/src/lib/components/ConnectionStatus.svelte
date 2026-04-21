<script lang="ts">
    import { gameStore } from '$lib/stores/game';
    import { page } from '$app/state';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import { connectWS, disconnectWS, leaveGame } from '$lib/api/websocket';
    import { fade } from 'svelte/transition';

    import * as Alert from '$lib/components/ui/alert';
    import Spinner from '$lib/components/ui/spinner/spinner.svelte';
    import Button from '$lib/components/ui/button/button.svelte';

    function handleRetry() {
        return;
    }

    function handleLeave() {
        leaveGame();
        gameStore.reset();
        goto(resolve('/'));
    }

    const connection = $derived($gameStore.connection);
</script>

{#if page.params.code && (connection.state === 'connecting' || connection.state === 'reconnecting' || connection.state === 'unreachable' || connection.state === 'invalid-session')}
    <div
        class="absolute inset-0 z-50 flex items-center justify-center bg-slate-950/60 backdrop-blur-sm"
        transition:fade={{ duration: 300 }}
    >
        <div class="max-w-fit">
            {#if connection.state === 'connecting'}
                <Alert.Root>
                    <div class="flex items-center justify-center gap-3">
                        <Spinner class="size-5" />
                        <Alert.Title>Connecting...</Alert.Title>
                    </div>
                </Alert.Root>
            {:else if connection.state === 'reconnecting'}
                <Alert.Root>
                    <div class="flex items-center justify-center gap-3">
                        <Spinner class="size-5" />
                        <Alert.Title>Reconnecting...</Alert.Title>
                    </div>
                    <Alert.Description class="text-center">
                        Attempt {connection.attempt} out of 10
                    </Alert.Description>
                </Alert.Root>
            {:else if connection.state === 'unreachable'}
                <Alert.Root variant="destructive">
                    <div class="flex flex-col items-center">
                        <Alert.Title class="text-center">Connection Lost</Alert.Title>
                        <Alert.Description class="text-center">
                            We couldn't reconnect to the game.<br />
                            Please check your internet connection and try again.
                        </Alert.Description>

                        <div class="mt-4 flex items-center gap-3">
                            <Button onclick={handleRetry}>Retry</Button>
                            <Button variant="secondary" onclick={handleLeave}>
                                Return to Lobby
                            </Button>
                        </div>
                    </div>
                </Alert.Root>
            {:else if connection.state === 'invalid-session'}
                <Alert.Root variant="destructive">
                    <Alert.Title>Session Not Found</Alert.Title>
                    <Alert.Description>
                        This game session no longer exists or has expired.
                    </Alert.Description>

                    <div class="mt-4 flex justify-center">
                        <Button onclick={handleLeave}>Return to Lobby</Button>
                    </div>
                </Alert.Root>
            {/if}
        </div>
    </div>
{/if}
