<script lang="ts">
    import { ApiError, createGame, joinGame } from '$lib/api/client';
    import { gameStore } from '$lib/stores/game';
    import { notificationStore } from '$lib/stores/notification';
    import { goto } from '$app/navigation';
    import Icon from '@iconify/svelte';
    import { resolve } from '$app/paths';
    import { onMount } from 'svelte';

    onMount(() => {
        gameStore.reset();
    });

    let game_code = $state('');
    let loading = $state(false);

    async function handleCreate(mode: 'ai' | 'multiplayer') {
        loading = true;
        gameStore.reset();

        try {
            const res = await createGame(mode);
            goto(resolve(`/game/${res.game_code}?player_token=${res.player_token}`));
        } catch {
            loading = false;
            notificationStore.push({
                title: 'Unable to create game',
                message: `We couldn’t reach the server.
                    Please check your internet connection or try again in a moment.`,
                type: 'error'
            });
        }
    }

    async function handleJoin() {
        if (!game_code) return;

        loading = true;
        gameStore.reset();

        try {
            const res = await joinGame(game_code);
            goto(resolve(`/game/${game_code}?player_token=${res.player_token}`));
        } catch (err) {
            loading = false;

            if (err instanceof ApiError) {
                if (err.code === 'session_not_found') {
                    notificationStore.push({
                        title: 'Session not found',
                        message: 'Check the game code and try again.',
                        type: 'error'
                    });
                } else if (err.code === 'game_full') {
                    notificationStore.push({
                        title: 'Game is already full',
                        message: 'The game you are trying to enter is already full.',
                        type: 'error'
                    });
                } else {
                    notificationStore.push({
                        title: 'Unable to create game',
                        message: `We couldn’t reach the server.
                            Please check your internet connection or try again in a moment.`,
                        type: 'error'
                    });
                }
            }
        }
    }
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-24">
    <!-- Hero section -->
    <section class="flex flex-col gap-6 text-center">
        <h1 class="text-6xl font-bold uppercase">
            BATTLESHIP <span class="text-primary">ONLINE</span>
        </h1>
        <p class="max-w-xl text-base text-foreground/75">
            A turn-based naval strategy game played on a hidden grid.
        </p>
    </section>

    <!-- Action Buttons -->
    <section class="grid w-full max-w-4xl grid-cols-2 gap-4 text-xs font-semibold tracking-wider">
        <!-- Play vs AI -->
        <button
            class="glass-panel flex cursor-pointer items-center justify-between border border-primary/20 px-8 py-6"
            onclick={() => handleCreate('ai')}
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="lucide:bot" class="h-6 w-6 text-primary/80" /> PLAY VS AI
            </div>
            <span>›</span>
        </button>

        <!-- Quick Match -->
        <button
            class="glass-panel flex cursor-not-allowed items-center justify-between border border-border/20 px-8 py-6"
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="mage:zap" class="h-6 w-6" />
                <div>
                    QUICK MATCH
                    <span class="text-muted-foreground">(COMING SOON)</span>
                </div>
            </div>
            <span class="text-base">›</span>
        </button>

        <!-- Host Game -->
        <button
            class="glass-panel flex cursor-pointer items-center justify-between border border-border/20 px-8 py-6"
            onclick={() => handleCreate('multiplayer')}
            disabled={loading}
        >
            <div class="flex items-center gap-4 text-left">
                <Icon icon="mdi:shield-plus-outline" class="h-6 w-6" /> HOST GAME
            </div>
            <span>›</span>
        </button>

        <!-- Join Game -->
        <div class="glass-panel flex items-center border border-border/20 px-8 py-6">
            <form
                onsubmit={(e) => {
                    e.preventDefault();
                    handleJoin();
                }}
                class="flex w-full items-center justify-between"
            >
                <div class="flex flex-1 items-center gap-4">
                    <Icon icon="mdi:key-outline" class="h-6 w-6" />
                    <input
                        bind:value={game_code}
                        placeholder="ENTER CODE"
                        maxlength="6"
                        class="w-full bg-transparent outline-none"
                    />
                </div>

                <button
                    type="submit"
                    disabled={loading}
                    class="ml-4 cursor-pointer text-xs tracking-wider text-primary/70"
                >
                    JOIN
                </button>
            </form>
        </div>
    </section>

    <!-- Info section -->
    <section
        class="grid w-full max-w-4xl grid-cols-3 gap-8 text-center text-xs text-muted-foreground"
    >
        <div class="flex flex-col gap-2 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">01 POSITIONING</div>
            <p class="text-[11px]">
                Place all ships on the grid. Ships cannot overlap or touch each other.
            </p>
        </div>

        <div class="flex flex-col gap-2 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">02 COMBAT</div>
            <p class="text-[11px]">
                Take turns firing at enemy coordinates. A hit grants another shot.
            </p>
        </div>

        <div class="flex flex-col gap-2 px-3 py-2">
            <div class="font-semibold tracking-widest uppercase">03 VICTORY</div>
            <p class="text-[11px]">Destroy all enemy ships to win the game.</p>
        </div>
    </section>
</div>
