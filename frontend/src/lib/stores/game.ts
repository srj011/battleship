import type { GameState, GameUpdate, ShipPlacement } from '$lib/types';
import { writable } from 'svelte/store';

type GameStore = {
    game: GameState | null;
    randomFleet: ShipPlacement[] | null;
    connected: boolean;
};

function createGameStore() {
    const { subscribe, update, set } = writable<GameStore>({
        game: null,
        randomFleet: null,
        connected: false
    });

    function applyGameUpdate(msg: GameUpdate) {
        update((s) => {
            if (!s.game) return s;

            const game = {
                ...s.game,
                turn: msg.turn,
                status: msg.status,
                player_board: msg.player_board,
                opponent_board: msg.opponent_board
            };

            const outcome = msg.event.outcome;
            const isSelf = msg.event.player === s.game.player;
            const key = isSelf ? 'opponent_fleet' : 'player_fleet';

            const fleet = game[key];

            game[key] = {
                ...fleet,
                ships: fleet.ships.map((ship) => {
                    let updated_ship = ship;
                    if (outcome.damage && ship.ship_type === outcome.damage.ship_type) {
                        updated_ship = { ...ship, damage: outcome.damage.total };
                    }
                    if (outcome.sunk_ship && ship.ship_type === outcome.sunk_ship) {
                        updated_ship = { ...ship, sunk: true };
                    }
                    return updated_ship;
                })
            };
            return { ...s, game };
        });
    }

    return {
        subscribe,
        setGame: (game: GameState) => update((s) => ({ ...s, game })),
        applyGameUpdate,
        setRandomFleet: (fleet: ShipPlacement[]) => update((s) => ({ ...s, randomFleet: fleet })),
        setConnected: (val: boolean) => update((s) => ({ ...s, connected: val })),
        reset: () => set({ game: null, randomFleet: null, connected: false })
    };
}

export const gameStore = createGameStore();
