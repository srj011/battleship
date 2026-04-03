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

    interface Props {
        board: BoardView | PreviewBoard;
        clickable?: boolean;
        onCellClick?: (coord: Coord) => void;
        onRightClick?: (coord: Coord) => void;
        onCellHover?: (coord: Coord) => void;
        isCellClickable?: (cell: CellView | PreviewCell) => boolean;
        class?: string;
    }

    const {
        board,
        clickable = false,
        onCellClick,
        onRightClick,
        onCellHover,
        isCellClickable = () => true,
        class: className = ''
    }: Props = $props();

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

    function getShipBorders(cells: (CellView | PreviewCell)[][], coord: Coord): string {
        const cell = cells[coord.row][coord.col];
        const ship = getShipType(cell);

        if (!ship) return '';

        const directions = [
            { dr: -1, dc: 0, cls: 'border-t-2' },
            { dr: 1, dc: 0, cls: 'border-b-2' },
            { dr: 0, dc: -1, cls: 'border-l-2' },
            { dr: 0, dc: 1, cls: 'border-r-2' }
        ];
        let classes = '';
        const { row, col } = coord;

        for (const { dr, dc, cls } of directions) {
            const nr = row + dr;
            const nc = col + dc;

            const adj_cell = cells[nr]?.[nc];
            const adj_ship = adj_cell ? getShipType(adj_cell) : null;

            if (adj_ship !== ship) {
                if (cell.type === 'ship' || cell.type === 'placed') {
                    classes += ` ${cls} border-indigo-600`;
                } else if (cell.type === 'hit' || cell.type === 'preview-invalid') {
                    classes += ` ${cls} border-red-600`;
                } else {
                    classes += ` ${cls} border-green-600`;
                }
            }
        }
        return classes;
    }

    function getGridBorder(cell: CellView | PreviewCell): string {
        const ship = getShipType(cell);

        if (ship) return '';

        return 'border border-sky-950/30';
    }

    function getShipType(cell: CellView | PreviewCell): ShipType | null {
        if ('ship_type' in cell) return cell.ship_type;
        return null;
    }
</script>

<div class={`grid grid-cols-[auto_repeat(10,2.5rem)] items-center ${className}`}>
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
                class={`h-10 w-10 focus:outline-none
                    ${getCellColor(cell)}
                    ${getGridBorder(cell)}
                    ${getShipBorders(board.cells, { row: rowIndex, col: colIndex })}
                    ${clickable && isCellClickable(cell) ? 'cursor-pointer' : ''}
                `}
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
