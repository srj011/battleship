<script lang="ts">
    import './layout.css';
    import favicon from '$lib/assets/favicon.svg';
    import { apiHealth } from '$lib/stores/app';
    import { checkHealth } from '$lib/api/client';
    import { onMount } from 'svelte';
    import { page } from '$app/state';

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
            <h1 class="text-lg font-semibold">Battleship</h1>

            {#if page.params.code}
                <span class="text-sm font-semibold">Game code: {page.params.code}</span>
            {/if}

            <div class="flex items-center gap-2 text-sm text-gray-300">
                <span>Status:</span>
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
    <main class="flex flex-1">
        <div class="mx-auto flex w-full max-w-5xl flex-col p-4">
            {@render children()}
        </div>
    </main>

    <!-- Footer -->
    <footer class="bg-gray-200 text-sm text-gray-500">
        <div class="mx-auto flex max-w-5xl justify-between px-4 py-3">
            <span>© Samved Rajesh 2026</span>
            <span>Built with Rust & Svelte</span>
        </div>
    </footer>
</div>
