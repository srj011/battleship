import type { ShipType } from '$lib/types';

export const BOARD_SIZE = 10;
export const SHIP_LENGTHS: Record<ShipType, number> = {
    carrier: 5,
    battleship: 4,
    destroyer: 3,
    submarine: 3,
    patrolboat: 2
};
export const TOTAL_SHIPS = Object.keys(SHIP_LENGTHS).length;
