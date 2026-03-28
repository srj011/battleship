<script lang="ts">
    import { createGame, joinGame } from '$lib/api/client';
    import { goto } from '$app/navigation';

    let game_code = $state('');
    let loading = $state(false);

    async function handleCreate(mode: 'ai' | 'multiplayer') {
        loading = true;

        const res = await createGame(mode);
        // eslint-disable-next-line svelte/no-navigation-without-resolve
        goto(`/game/${res.game_code}?player_token=${res.player_token}`);
    }

    async function handleJoin() {
        if (!game_code) return;

        loading = true;

        const res = await joinGame(game_code);

        // eslint-disable-next-line svelte/no-navigation-without-resolve
        goto(`/game/${game_code}?player_token=${res.player_token}`);
    }
</script>

<div class="flex min-h-screen flex-col items-center justify-center gap-6">
    <h1 class="text-4xl font-bold">Battleship</h1>

    <div class="flex gap-4">
        <button onclick={() => handleCreate('ai')} disabled={loading} style:cursor="pointer">
            Play vs AI
        </button>
        <button
            onclick={() => handleCreate('multiplayer')}
            disabled={loading}
            style:cursor="pointer"
        >
            Create Multiplayer
        </button>
    </div>

    <div class="flex gap-2">
        <input bind:value={game_code} placeholder="Enter game code" class="border px-3 py-2" />

        <button onclick={handleJoin} disabled={loading} style:cursor="pointer"> Join </button>
    </div>
</div>
