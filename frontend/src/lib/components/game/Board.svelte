<script lang="ts">
    import type { BoardView, CellView, Coord, PreviewBoard, PreviewCell } from '$lib/types';

    const COL_LABELS = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    interface Props {
        board: BoardView | PreviewBoard;
        clickable?: boolean;
        onCellClick?: (coord: Coord) => void;
        onRightClick?: (coord: Coord) => void;
        isCellClickable?: (cell: CellView | PreviewCell) => boolean;
        onPointerUp?: () => void;
        onPointerDown?: (coord: Coord) => void;
        onPointerEnter?: (coord: Coord) => void;
        onPointerLeave?: () => void;
        class?: string;
    }

    const {
        board,
        clickable = false,
        onCellClick,
        onRightClick,
        isCellClickable = () => true,
        onPointerUp,
        onPointerDown,
        onPointerEnter,
        onPointerLeave,
        class: className = ''
    }: Props = $props();

    function handleClick(coord: Coord) {
        if (!clickable || !onCellClick) return;
        onCellClick(coord);
    }

    function getCellClass(cell: CellView | PreviewCell): string {
        switch (cell.type) {
            case 'ship':
            case 'placed':
                return 'ship-cell';
            case 'hit':
            case 'preview-invalid':
                return 'hit-cell';
            case 'miss':
                return 'miss-cell';
            case 'blocked':
                return 'blocked-cell';
            case 'preview-valid':
                return 'bg-board-ship/25 border-board-ship-border/50';
            default:
                return 'bg-board-water';
        }
    }
</script>

<div
    class={`grid grid-cols-[auto_repeat(10,3.2rem)] items-center ${className}`}
    onpointerleave={onPointerLeave}
    role="application"
    aria-label="Game board"
    tabindex="-1"
>
    <!-- Empty left corner space -->
    <div></div>

    <!-- Column Label -->
    {#each COL_LABELS as label (label)}
        <div class="mb-2 text-center text-[0.6rem] text-muted-foreground">{label}</div>
    {/each}
    {#each board.cells as row, rowIndex (rowIndex)}
        <!-- Row Label -->
        <div class="mr-2 text-center text-[0.6rem] text-muted-foreground">{rowIndex + 1}</div>

        {#each row as cell, colIndex (`${rowIndex}-${colIndex}`)}
            <button
                class={`m-0.5 h-12 w-12 border
                    ${getCellClass(cell)}
                    ${clickable && isCellClickable(cell) ? 'cursor-pointer' : ''}
                `}
                disabled={!clickable || !isCellClickable(cell)}
                aria-label={`Cell ${rowIndex}, ${colIndex} - ${cell.type}`}
                oncontextmenu={(e) => {
                    e.preventDefault();
                    onRightClick?.({ row: rowIndex, col: colIndex });
                }}
                onclick={() => handleClick({ row: rowIndex, col: colIndex })}
                onpointerenter={() => onPointerEnter?.({ row: rowIndex, col: colIndex })}
                onpointerup={() => onPointerUp?.()}
                onpointerdown={() => onPointerDown?.({ row: rowIndex, col: colIndex })}
            ></button>
        {/each}
    {/each}
</div>
