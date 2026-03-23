<script lang="ts">
    import type { BoardView as BoardType, CellView, Coord } from "$lib/types";

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
                return "bg-blue-200";
            case 'empty':
                return "bg-blue-200";
            case 'ship':
                return "bg-green-400";
            case 'hit':
                return "bg-red-500";
            case 'miss':
                return "bg-gray-400";
        }
    }
</script>

<div class="grid grid-cols-10 gap-1">
    {#each board.cells as row, rowIndex (rowIndex)}
        {#each row as cell, colIndex (`${rowIndex}-${colIndex}`)}
            <button
            class={`w-8 h-8 border cursor-pointer ${getCellClass(cell)}`}
            aria-label={`Cell ${rowIndex}, ${colIndex} - ${typeof cell === 'object' ? 'ship' : cell}`}
            onclick={() => handleClick({
                coord: {row: rowIndex, col: colIndex}
            })}></button>
        {/each}
    {/each}
</div>
