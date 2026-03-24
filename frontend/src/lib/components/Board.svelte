<script lang="ts">
    import type { BoardView as BoardType, CellView, Coord } from "$lib/types";

    const COL_LABELS = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

    let { board, clickable=false, onCellClick } = $props<{
        board: BoardType;
        clickable?: boolean;
        onCellClick?: (args: {coord: Coord}) => void;
    }>();

    function handleClick(args: {coord: Coord}) {
        if (!clickable || !onCellClick) return;
        onCellClick(args);
    }

    function getCellClass(cell: CellView) {
        switch (cell.type) {
            case 'unknown':
                return "bg-sky-100";
            case 'empty':
                return "bg-sky-100";
            case 'ship':
                return "bg-indigo-500";
            case 'hit':
                return "bg-red-500";
            case 'miss':
                return "bg-gray-400";
        }
    }

    function isCellClickable(cell: CellView) {
        return cell.type === 'unknown';
    }
</script>

<div class="grid grid-cols-[auto_repeat(10,2.5rem)] items-center">

    <!-- Empty left corner space -->
    <div></div>

    <!-- Column Label -->
    {#each COL_LABELS as label (label)}
        <div class="text-xs text-center mb-2">{label}</div>
    {/each}
    {#each board.cells as row, rowIndex (rowIndex)}

    <!-- Row Label -->
    <div class="text-xs text-center mr-2">{rowIndex + 1}</div>

        {#each row as cell, colIndex (`${rowIndex}-${colIndex}`)}
            <button
            class={`w-10 h-10 border border-sky-950/50 ${
                clickable && isCellClickable(cell)
                ? "cursor-pointer"
                : ""
            } ${getCellClass(cell)}`}
            disabled={!clickable || !isCellClickable(cell)}
            aria-label={`Cell ${rowIndex}, ${colIndex} - ${cell.type}`}
            onclick={() => handleClick({
                coord: {row: rowIndex, col: colIndex}
            })}></button>
        {/each}
    {/each}
</div>
