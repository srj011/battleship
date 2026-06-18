<script lang="ts">
    import Board from '$lib/components/game/Board.svelte';
    import Fleet from '$lib/components/game/Fleet.svelte';
    import type { BoardView, CellView, Coord, FleetView, PreviewCell } from '$lib/types';

    interface Props {
        variant: 'player' | 'opponent';
        board: BoardView;
        fleet: FleetView;
        dimmed?: boolean;
        clickable?: boolean;
        onCellClick?: (coord: Coord) => void;
        isCellClickable?: (cell: CellView | PreviewCell) => boolean;
    }

    const {
        variant,
        board,
        fleet,
        dimmed = false,
        clickable = false,
        onCellClick,
        isCellClickable
    }: Props = $props();
</script>

<div
    class="flex flex-col gap-8 rounded-xl border border-white/5 bg-surface/30 p-4 backdrop-blur-md"
>
    <h2
        class="relative text-center font-bold tracking-wide uppercase
        after:absolute after:top-full after:left-1/2 after:mt-2
                   after:h-px after:w-10 after:-translate-x-1/2
                   after:bg-primary/50 after:content-['']"
    >
        {variant === 'player' ? 'YOUR FLEET' : 'ENEMY WATERS'}
    </h2>
    <!-- Player's Board -->
    <Board
        {board}
        class={`${dimmed ? 'opacity-70' : ''}`}
        {clickable}
        {onCellClick}
        {isCellClickable}
    />

    <!-- Player's fleet -->
    {#if fleet}
        <Fleet {fleet} {variant} />
    {/if}
</div>
