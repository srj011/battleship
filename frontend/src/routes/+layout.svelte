<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { apiHealth } from '$lib/stores/app';
    import { gameStore } from '$lib/stores/game';
    import { checkHealth } from '$lib/api/client';
    import { onMount } from 'svelte';
    import { page } from '$app/state';
    import Icon from '@iconify/svelte';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import Logo from '$lib/components/Logo.svelte';
    import ConnectionStatus from '$lib/components/ConnectionStatus.svelte';
    import NotificationStack from '$lib/components/notification/NotificationStack.svelte';
    import LeaveGame from '$lib/components/LeaveGame.svelte';
    import { Button } from '$lib/components/ui/button';

    const { children } = $props();

    onMount(() => {
        checkHealth();

        const interval = setInterval(() => checkHealth(), 5000);
        return () => clearInterval(interval);
    });

    const phase = $derived($gameStore.game?.status.type);

    let copied = $state(false);

    const opponent_ready = $derived($gameStore.game?.opponent_ready);
    const opponent_present = $derived($gameStore.game?.opponent_present);

    function copyGameLink() {
        const value = page.params.code ?? '';
        navigator.clipboard.writeText(value);

        copied = true;
        setTimeout(() => (copied = false), 1500);
    }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col">
    <!-- Header -->
    <header class="bg-gray-900 text-white">
        <div class="grid grid-cols-3 items-center px-6 py-3">
            <!-- Left -->
            <div class="flex items-center gap-4">
                <Logo />

                {#if page.params.code}
                    <!-- Divider -->
                    <div class="h-3 w-px bg-white/20"></div>

                    <div class="flex gap-1 text-xs font-semibold tracking-wider text-gray-400">
                        PHASE:
                        <span>
                            {phase === 'placing_ships'
                                ? 'SHIP PLACEMENT'
                                : phase === 'ongoing'
                                  ? 'ONGOING'
                                  : phase === 'abandoned'
                                    ? 'ABANDONED'
                                    : 'FINISHED'}
                        </span>
                    </div>
                {/if}
            </div>

            <!-- Center -->
            <div class="items-center justify-self-center">
                {#if page.params.code}
                    <button
                        class="flex cursor-pointer items-center gap-1
                        border border-neutral-600 px-3 py-1
                        text-xs tracking-widest select-none"
                        onclick={copyGameLink}
                        disabled={copied}
                        title="Click to Copy"
                    >
                        <span class="opacity-60">CODE:</span>
                        <span class="font-mono text-sm">{copied ? 'COPIED' : page.params.code}</span
                        >
                        <Icon icon="material-symbols:content-copy-rounded" width="16" height="16" />
                    </button>
                {/if}
            </div>

            <!-- Right -->
            <div class="flex items-center gap-10 justify-self-end">
                <div class="flex items-center gap-3 text-xs tracking-wider">
                    <div class="flex items-center gap-2 font-semibold text-gray-400 uppercase">
                        {#if page.params.code}
                            {#if opponent_present}
                                {#if opponent_ready}
                                    <Icon
                                        icon="material-symbols:check-circle-rounded"
                                        width="15"
                                        height="15"
                                    />
                                    <span>OPPONENT: READY</span>
                                {:else}
                                    <Icon icon="mdi:hourglass" width="15" />
                                    <span>OPPONENT: PLACING</span>
                                {/if}
                            {:else}
                                <Icon icon="mdi:hourglass" width="15" />
                                {#if $gameStore.game?.status.type === 'placing_ships'}
                                    <span>OPPONENT: NOT JOINED</span>
                                {:else}
                                    <span>OPPONENT: LEFT</span>
                                {/if}
                            {/if}
                        {/if}
                    </div>

                    {#if page.params.code}
                        <!-- Divider -->
                        <div class="h-4 w-px bg-white/20"></div>
                    {/if}

                    <div class="flex items-center gap-1 font-semibold text-gray-300 uppercase">
                        <Icon icon="fluent-mdl2:status-circle-sync" font-size="25" />
                        <span>SERVER:</span>
                        <span
                            >{$apiHealth === 'checking'
                                ? 'CHECKING'
                                : $apiHealth === 'online'
                                  ? 'ONLINE'
                                  : 'OFFLINE'}</span
                        >
                    </div>
                </div>
                {#if page.params.code}
                    {#if $gameStore.game?.status.type === 'placing_ships' || $gameStore.game?.status.type === 'ongoing'}
                        <LeaveGame />
                    {:else}
                        <Button
                            onclick={() => {
                                gameStore.reset();
                                goto(resolve('/'));
                            }}
                        >
                            Return to Lobby
                        </Button>
                    {/if}
                {/if}
            </div>
        </div>
    </header>

    <!-- Main -->
    <main class="flex flex-1 p-4">
        <NotificationStack />
        {@render children()}
        <ConnectionStatus />
    </main>

    <!-- Footer -->
    <footer class="flex items-center justify-center gap-4 border-t border-neutral-600 py-3 text-sm">
        <div class="flex items-center gap-4">
            <span>© 2026 Battleship</span>
            <span>•</span>
            <span>Built with Rust & Svelte</span>
            <span>•</span>
            <a
                href="https://github.com/srj011/battleship"
                target="_blank"
                rel="noopener noreferrer"
            >
                <Icon icon="mdi:github" class="h-5 w-5" />
            </a>
        </div>
    </footer>
</div>
