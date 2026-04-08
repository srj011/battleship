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

    const { children } = $props();

    onMount(() => {
        checkHealth();
    });

    let copied = $state(false);

    function copyGameLink() {
        const value = page.params.code ?? '';
        navigator.clipboard.writeText(value);

        copied = true;
        setTimeout(() => (copied = false), 1500);
    }

    function handleLeave() {
        gameStore.reset();
        goto(resolve('/'));
    }
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col">
    <!-- Header -->
    <header class="bg-gray-900 text-white">
        <div class="grid grid-cols-3 items-center px-6 py-3">
            <!-- Left -->
            <h1 class="justify-self-start text-lg font-semibold uppercase">BATTLESHIP</h1>

            <!-- Center -->
            <div class="items-center justify-self-center">
                {#if page.params.code}
                    <button
                        class="flex cursor-pointer items-center gap-1
                        border border-neutral-600 px-3 py-1
                        text-xs tracking-widest uppercase select-none"
                        onclick={copyGameLink}
                        disabled={copied}
                        title="Click to Copy"
                    >
                        <span class="opacity-60">CODE:</span>
                        <span class="font-mono text-sm">{copied ? 'COPIED' : page.params.code}</span
                        >
                        <Icon icon="material-symbols:content-copy-rounded" class="h-4 w-4" />
                    </button>
                {/if}
            </div>

            <!-- Right -->
            <div class="flex items-center gap-3 justify-self-end">
                {#if page.params.code}
                    <button
                        class="cursor-pointer border border-red-500/50 px-3 py-2
                        text-xs font-semibold text-red-500 uppercase"
                        onclick={handleLeave}
                    >
                        LEAVE GAME
                    </button>
                {/if}
                <div class="flex items-center gap-2 text-sm text-gray-300 uppercase">
                    <span class="text-[0.8rem] font-semibold">STATUS:</span>
                    <div
                        title={$apiHealth}
                        class={`h-3 w-3 rounded-full ${
                            $apiHealth === 'checking'
                                ? 'bg-yellow-400'
                                : $apiHealth === 'online'
                                  ? 'bg-green-500'
                                  : 'bg-red-500'
                        }`}
                    ></div>
                </div>
            </div>
        </div>
    </header>

    <!-- Main -->
    <main class="flex flex-1 p-4">
        {@render children()}
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
