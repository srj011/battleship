<script lang="ts">
    import { sendWS } from '$lib/api/websocket';
    import { gameStore } from '$lib/stores/game';
    import PlayerPanel from '$lib/components/game/PlayerPanel.svelte';
    import type { CellView, Coord, FireMessage, PreviewCell } from '$lib/types';

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
            <PlayerPanel
                variant="player"
                board={$gameStore.game.player_board}
                fleet={$gameStore.game.player_fleet}
                dimmed={isMyTurn}
            />

            <div
                class="w-px flex-1 bg-linear-to-b from-transparent via-primary/10 to-transparent"
            ></div>

            <!-- Opponent's section-->
            <PlayerPanel
                variant="opponent"
                board={$gameStore.game.opponent_board}
                fleet={$gameStore.game.opponent_fleet}
                dimmed={!isMyTurn}
                clickable={isMyTurn}
                onCellClick={handleFire}
                {isCellClickable}
            />
        </div>
    {/if}
</div>
