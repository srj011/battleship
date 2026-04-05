<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { apiHealth } from '$lib/stores/app';
    import { checkHealth } from '$lib/api/client';
    import { onMount } from 'svelte';
    import { page } from '$app/state';
    import Icon from '@iconify/svelte';

    const { children } = $props();

    onMount(() => {
        checkHealth();
    });
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex min-h-screen flex-col">
    <!-- Header -->
    <header class="bg-gray-900 text-white">
        <div class="justify mx-auto flex max-w-5xl items-center justify-between px-4 py-3">
            <h1 class="text-lg font-semibold uppercase">BATTLESHIP</h1>

            {#if page.params.code}
                <span class="text-sm font-semibold">Game code: {page.params.code}</span>
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
    </header>

    <!-- Main -->
    <main class="mx-auto flex w-full max-w-5xl flex-1 flex-col p-4">
        {@render children()}
    </main>

    <!-- Footer -->
    <footer class="flex flex-col items-center gap-2 py-8 text-sm">
        <div class="flex items-center gap-4">
            <span>© 2026 Battleship</span>
            <a
                href="https://github.com/srj011/battleship"
                target="_blank"
                rel="noopener noreferrer"
            >
                <Icon icon="mdi:github" class="h-5 w-5" />
            </a>
        </div>

        <div class="text-xs">Built with Rust & Svelte</div>
    </footer>
</div>
