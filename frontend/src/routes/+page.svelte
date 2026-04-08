<script lang="ts">
    import { createGame, joinGame } from '$lib/api/client';
    import { gameStore } from '$lib/stores/game';
    import { goto } from '$app/navigation';
    import Icon from '@iconify/svelte';
    import { resolve } from '$app/paths';

    let game_code = $state('');
    let loading = $state(false);

    async function handleCreate(mode: 'ai' | 'multiplayer') {
        loading = true;
        gameStore.reset();

        const res = await createGame(mode);

        goto(resolve(`/game/${res.game_code}?player_token=${res.player_token}`));
    }

    async function handleJoin() {
        if (!game_code) return;

        loading = true;
        gameStore.reset();

        const res = await joinGame(game_code);

        goto(resolve(`/game/${game_code}?player_token=${res.player_token}`));
    }
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-20">
    <!-- Hero section -->
    <section class="flex flex-col gap-6 text-center">
        <h1 class="text-6xl font-bold uppercase">
            BATTLESHIP <span>ONLINE</span>
        </h1>
        <p class="max-w-xl text-base">A turn-based naval strategy game played on a hidden grid.</p>
    </section>

    <!-- Action Buttons -->
    <section class="grid w-full max-w-3xl grid-cols-2 gap-4">
        <!-- Play vs AI -->
        <button
            class="flex cursor-pointer items-center justify-between border px-6 py-5"
            onclick={() => handleCreate('ai')}
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="bxs:bot" class="h-6 w-6" /> PLAY VS AI
            </div>
            <span>›</span>
        </button>

        <!-- Quick Match -->
        <button
            class="flex cursor-pointer items-center justify-between border px-6 py-5"
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="mage:zap-fill" class="h-6 w-6" /> QUICK MATCH (Coming Soon)
            </div>
            <span>›</span>
        </button>

        <!-- Host Game -->
        <button
            class="flex cursor-pointer items-center justify-between border px-6 py-5"
            onclick={() => handleCreate('multiplayer')}
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="mdi:shield-plus" class="h-6 w-6" /> HOST GAME
            </div>
            <span>›</span>
        </button>

        <!-- Join Game -->
        <div class="flex items-center border px-6 py-5">
            <form
                onsubmit={(e) => {
                    e.preventDefault();
                    handleJoin();
                }}
                class="flex w-full items-center justify-between"
            >
                <div class="flex flex-1 items-center gap-4">
                    <Icon icon="mdi:key" class="h-6 w-6" />
                    <input
                        bind:value={game_code}
                        placeholder="ENTER CODE"
                        maxlength="6"
                        class="w-full bg-transparent outline-none"
                    />
                </div>

                <button type="submit" disabled={loading} class="ml-4 cursor-pointer text-sm">
                    JOIN
                </button>
            </form>
        </div>
    </section>

    <!-- Info section -->
    <section class="grid w-full max-w-5xl grid-cols-3 gap-12 text-sm">
        <div class="flex flex-col gap-2 border-l border-neutral-800 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">01 POSITIONING</div>
            <p>Place all ships on the grid. Ships cannot overlap or touch each other.</p>
        </div>

        <div class="flex flex-col gap-2 border-l border-neutral-800 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">02 COMBAT</div>
            <p>Take turns firing at enemy coordinates. A hit grants another shot.</p>
        </div>

        <div class="flex flex-col gap-2 border-l border-neutral-800 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">03 VICTORY</div>
            <p>Destroy all enemy ships to win the game.</p>
        </div>
    </section>
</div>
