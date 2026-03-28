import type { Coord } from '$lib/types';
import { BOARD_SIZE } from './config';

export function isWithinBounds(coord: Coord): boolean {
    return coord.row >= 0 && coord.col >= 0 && coord.row < BOARD_SIZE && coord.col < BOARD_SIZE;
}
