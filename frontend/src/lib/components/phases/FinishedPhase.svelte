<script lang="ts">
    import { gameStore } from '$lib/stores/game';
    import { notificationStore } from '$lib/stores/notification';
    import { sendWS } from '$lib/api/websocket';
    import Board from '$lib/components/game/Board.svelte';
    import Fleet from '$lib/components/game/Fleet.svelte';
    import type { ClientMessage, Notification } from '$lib/types';
    import Icon from '@iconify/svelte';
    import RematchDialog from '$lib/components/RematchDialog.svelte';

    const playerFleet = $derived($gameStore.game?.player_fleet);
    const opponentFleet = $derived($gameStore.game?.opponent_fleet);

    const isWin =
        $gameStore.game?.status.type === 'finished' &&
        $gameStore.game.status.winner === $gameStore.game.player;
    const isAbandoned = $gameStore.game?.status.type === 'abandoned';

    const opponent_present = $derived($gameStore.game?.opponent_present);
    const opponentDisconnected = $derived(
        $gameStore.playerDisconnect &&
            $gameStore.playerDisconnect.player !== $gameStore.game?.player
    );

    $effect(() => {
        if (!opponent_present) {
            const notification: Omit<Notification, 'id'> = {
                title: 'Opponent left',
                type: 'info'
            };
            notificationStore.push(notification);
        }
    });

    function handleRematch() {
        const msg: ClientMessage = { type: 'request_rematch' };
        sendWS(msg);
    }
</script>

<div class="flex flex-col items-center gap-6">
    {#if $gameStore.game}
        <RematchDialog />
        <!-- GameOver Status -->
        {#if $gameStore.game.status.type === 'finished' || isAbandoned}
            <div class="flex w-full flex-col items-center gap-4">
                <div
                    class={`flex w-full items-center justify-center gap-3 rounded-xs py-4 shadow-lg ${
                        isAbandoned ? 'bg-gray-500/90' : isWin ? 'bg-green-700/90' : 'bg-red-700/90'
                    }`}
                >
                    <Icon
                        icon={isAbandoned
                            ? 'mdi:exit-run'
                            : isWin
                              ? 'noto:trophy'
                              : 'emojione:skull-and-crossbones'}
                        width="25"
                        height="25"
                    />
                    <span
                        class="text-center text-xl font-semibold tracking-widest text-white uppercase"
                    >
                        {isAbandoned ? 'Opponent left the match' : isWin ? 'VICTORY' : 'DEFEAT'}
                    </span>
                </div>
                {#if !isAbandoned}
                    <button
                        class={`cursor-pointer rounded-sm px-4
                        py-2 text-sm font-semibold text-white uppercase
                        ${opponentDisconnected ? 'bg-gray-900/50' : 'bg-gray-900/90  hover:bg-gray-800/80'}`}
                        hidden={!opponent_present}
                        onclick={handleRematch}
                        disabled={$gameStore.game.rematch_state.type === 'requested' ||
                            opponentDisconnected}
                    >
                        Play Again
                    </button>
                {/if}
            </div>
        {/if}

        <div class="flex items-stretch gap-12">
            <!-- Player's section -->
            <div class="flex flex-col gap-8 p-2">
                <h2 class="text-center text-base font-bold tracking-wide uppercase">YOUR FLEET</h2>
                <!-- Player's Board -->
                <Board board={$gameStore.game.player_board} />

                <!-- Player's fleet -->
                {#if playerFleet}
                    <Fleet fleet={playerFleet} variant="player" />
                {/if}
            </div>

            <!-- VS separator -->
            <div class="relative flex items-center justify-center self-stretch">
                <!-- Full vertical line -->
                <div class="absolute inset-y-0 w-0.5 bg-neutral-300/30"></div>

                <!-- VS text -->
                <span
                    class="z-10 bg-white px-2 text-sm tracking-widest text-neutral-600/80 uppercase"
                >
                    VS
                </span>
            </div>

            <!-- Opponent's section-->
            <div class="flex flex-col gap-8 p-2">
                <h2 class="text-center text-base font-bold tracking-wide uppercase">
                    ENEMY WATERS
                </h2>
                <!-- Opponent's Board -->
                <Board board={$gameStore.game.opponent_board} />

                <!-- Opponent's fleet -->
                {#if opponentFleet}
                    <Fleet fleet={opponentFleet} variant="opponent" />
                {/if}
            </div>
        </div>
    {/if}
</div>
