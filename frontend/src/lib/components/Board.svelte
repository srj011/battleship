<script lang="ts">
    import type {
        BoardView,
        CellView,
        Coord,
        PreviewBoard,
        PreviewCell,
        ShipType
    } from '$lib/types';

    const COL_LABELS = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    const {
        board,
        clickable = false,
        onCellClick,
        onRightClick,
        onCellHover,
        isCellClickable
    } = $props<{
        board: BoardView | PreviewBoard;
        clickable?: boolean;
        onCellClick?: (coord: Coord) => void;
        onRightClick?: (coord: Coord) => void;
        onCellHover?: (coord: Coord) => void;
        isCellClickable?: (cell: CellView | PreviewCell) => boolean;
    }>();

    function handleClick(coord: Coord) {
        if (!clickable || !onCellClick) return;
        onCellClick(coord);
    }

    function getCellColor(cell: CellView | PreviewCell) {
        switch (cell.type) {
            case 'unknown':
            case 'empty':
                return 'bg-sky-100';
            case 'ship':
                return 'bg-indigo-500/90';
            case 'hit':
                return 'bg-red-500';
            case 'miss':
                return 'bg-gray-400/75';
            case 'blocked':
                return 'bg-gray-400';
            // Preview cells
            case 'placed':
                return 'bg-indigo-500/90';
            case 'preview-valid':
                return 'bg-green-400';
            case 'preview-invalid':
                return 'bg-red-400';
        }
    }

</script>

<div class="grid grid-cols-[auto_repeat(10,2.5rem)] items-center">
    <!-- Empty left corner space -->
    <div></div>

    <!-- Column Label -->
    {#each COL_LABELS as label (label)}
        <div class="mb-2 text-center text-xs">{label}</div>
    {/each}
    {#each board.cells as row, rowIndex (rowIndex)}
        <!-- Row Label -->
        <div class="mr-2 text-center text-xs">{rowIndex + 1}</div>

        {#each row as cell, colIndex (`${rowIndex}-${colIndex}`)}
            <button
            class={`w-10 h-10 border border-sky-950/50 ${
                clickable && isCellClickable(cell)
                ? "cursor-pointer"
                : ""
            } ${getCellColor(cell)}`}
            disabled={!clickable || !isCellClickable(cell)}
            aria-label={`Cell ${rowIndex}, ${colIndex} - ${cell.type}`}
                onclick={() => handleClick({ row: rowIndex, col: colIndex })}
                oncontextmenu={(e) => {
                    e.preventDefault();
                    onRightClick?.({ row: rowIndex, col: colIndex });
                }}
                onmouseenter={() => onCellHover?.({ row: rowIndex, col: colIndex })}
            ></button>
        {/each}
    {/each}
</div>
