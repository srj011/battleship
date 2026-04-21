import type {
    GameState,
    GameUpdate,
    ShipPlacement,
    Connection,
    Event
} from '$lib/types';
import { writable } from 'svelte/store';

type GameStore = {
    game: GameState | null;
    randomFleet: ShipPlacement[] | null;
    connection: Connection;
};

function createGameStore() {
    const { subscribe, update, set } = writable<GameStore>({
        game: null,
        randomFleet: null,
        connection: { state: 'idle', attempt: 0 },
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

    function connectionReducer(state: Connection, event: Event): Connection {
        switch (state.state) {
            case 'idle':
                if (event.type === 'CONNECT') {
                    return { state: 'connecting', attempt: 0 };
                }
                return state;

            case 'connecting':
                switch (event.type) {
                    case 'CONNECTED':
                        return { state: 'connected', attempt: 0 };
                    case 'INVALID_SESSION':
                        return { state: 'invalid-session', attempt: 0 };
                    case 'DISCONNECTED':
                        return { state: 'reconnecting', attempt: 0 };
                    default:
                        return state;
                }

            case 'connected':
                if (event.type === 'DISCONNECTED') {
                    return { state: 'reconnecting', attempt: 0 };
                }
                if (event.type === 'LEAVE') {
                    return { state: 'idle', attempt: 0 };
                }
                return state;

            case 'reconnecting':
                switch (event.type) {
                    case 'CONNECTED':
                        return { state: 'connected', attempt: 0 };
                    case 'RETRY':
                        return {
                            state: 'reconnecting',
                            attempt: state.attempt + 1
                        };
                    case 'MAX_RETRIES':
                        return { state: 'unreachable', attempt: state.attempt };
                    case 'LEAVE':
                        return { state: 'idle', attempt: 0 };
                    default:
                        return state;
                }

            case 'unreachable':
                if (event.type === 'RETRY') {
                    return { state: 'reconnecting', attempt: 0 };
                }
                if (event.type === 'LEAVE') {
                    return { state: 'idle', attempt: 0 };
                }
                return state;

            case 'invalid-session':
                if (event.type === 'LEAVE') {
                    return { state: 'idle', attempt: 0 };
                }
                return state;
        }
    }

    function dispatch(event: Event) {
        update((s) => ({
            ...s,
            connection: connectionReducer(s.connection, event)
        }));
    }

    return {
        subscribe,
        dispatch,
        setGame: (game: GameState) => update((s) => ({ ...s, game })),
        applyGameUpdate,
        setRandomFleet: (fleet: ShipPlacement[]) => update((s) => ({ ...s, randomFleet: fleet })),
        reset: () =>
            set({
                game: null,
                randomFleet: null,
                connection: { state: 'idle', attempt: 0 },
            })
    };
}

export const gameStore = createGameStore();
