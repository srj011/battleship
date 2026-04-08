<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/Board.svelte';
    import Fleet from '$lib/components/Fleet.svelte';
    import type { CellView, Coord, FireMessage, PreviewCell } from '$lib/types';

    const playerFleet = $derived($gameStore.game?.player_fleet);
    const opponentFleet = $derived($gameStore.game?.opponent_fleet);

    const isMyTurn = $derived(
        $gameStore.game?.status.type === 'ongoing' &&
            $gameStore.game.player !== null &&
            $gameStore.game.player === $gameStore.game.turn
    );

    function handleFire(coord: Coord) {
        if (!isMyTurn) return;

        const msg: FireMessage = {
            type: 'fire',
            coord
        };
        sendWS(msg);
    }

    function isCellClickable(cell: CellView | PreviewCell): boolean {
        return cell.type === 'empty' || cell.type === 'unknown';
    }
</script>

<div class="flex flex-col items-center gap-8">
    {#if $gameStore.game}
        <!-- Turn Indicator -->
        <div class="mx-auto max-w-md rounded bg-gray-200 px-4 py-3 text-sm text-black">
            <div class="flex items-center justify-center">
                <span class="font-semibold tracking-widest uppercase">
                    {isMyTurn ? 'Your Turn' : "Opponent's Turn"}
                </span>
            </div>
        </div>

        <div class="flex items-stretch gap-12">
            <!-- Player's section -->
            <div class="flex flex-col gap-8 p-2">
                <h2 class="text-center text-base font-bold tracking-wide uppercase">YOUR FLEET</h2>
                <!-- Player's Board -->
                <Board
                    class={`${isMyTurn ? 'opacity-60' : ''}`}
                    board={$gameStore.game.player_board}
                />

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
                <Board
                    class={`${!isMyTurn ? 'opacity-60' : ''}`}
                    board={$gameStore.game.opponent_board}
                    clickable={isMyTurn}
                    onCellClick={handleFire}
                    {isCellClickable}
                />

                <!-- Opponent's fleet -->
                {#if opponentFleet}
                    <Fleet fleet={opponentFleet} variant="opponent" />
                {/if}
            </div>
        </div>
    {/if}
</div>
