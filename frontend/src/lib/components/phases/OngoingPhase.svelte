<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import Board from '$lib/components/game/Board.svelte';
    import Fleet from '$lib/components/game/Fleet.svelte';
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
        <div class="flex flex-col items-center">
            <div
                class={`relative rounded-md border bg-card/60 px-8 py-2.5
                    before:absolute before:top-0 before:right-4 before:left-4
                    before:h-px before:content-['']
                    ${isMyTurn ? 'before:bg-primary/80' : 'before:bg-foreground/20'}`}
            >
                <span
                    class={`text-sm font-semibold tracking-[0.2em] uppercase
                    ${isMyTurn ? 'text-primary' : 'text-foreground/80'}`}
                >
                    ▶ {isMyTurn ? 'Your Turn' : "Opponent's Turn"}
                </span>
            </div>
        </div>

        <div class="flex items-stretch gap-16">
            <!-- Player's section -->
            <div
                class="flex flex-col gap-8 rounded-xl border border-white/5 bg-card/30 p-4 backdrop-blur-md"
            >
                <h2
                    class="relative text-center font-bold tracking-wide uppercase
                    after:absolute after:top-full after:left-1/2 after:mt-2
                               after:h-px after:w-10 after:-translate-x-1/2
                               after:bg-primary/50 after:content-['']"
                >
                    YOUR FLEET
                </h2>
                <!-- Player's Board -->
                <Board
                    class={`${isMyTurn ? 'opacity-70' : ''}`}
                    board={$gameStore.game.player_board}
                />

                <!-- Player's fleet -->
                {#if playerFleet}
                    <Fleet fleet={playerFleet} variant="player" />
                {/if}
            </div>

            <div
                class="w-px flex-1 bg-linear-to-b from-transparent via-primary/10 to-transparent"
            ></div>

            <!-- Opponent's section-->
            <div
                class="flex flex-col gap-8 rounded-xl border border-white/5 bg-surface/30 p-4 backdrop-blur-md"
            >
                <h2
                    class="relative text-center font-bold tracking-wide uppercase
                    after:absolute after:top-full after:left-1/2 after:mt-2
                               after:h-px after:w-10 after:-translate-x-1/2
                               after:bg-primary/50 after:content-['']"
                >
                    ENEMY WATERS
                </h2>
                <!-- Opponent's Board -->
                <Board
                    class={`${!isMyTurn ? 'opacity-70' : ''}`}
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
