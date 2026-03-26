import type { GameSnapshot, Player, ShipPlacement } from '$lib/types';
import { writable } from 'svelte/store';

type GameStore = {
    game: GameSnapshot | null;
    player: Player | null;
    randomFleet: ShipPlacement[] | null;
    connected: boolean;
};

function createGameStore() {
    const { subscribe, update, set } = writable<GameStore>({
        game: null,
        player: null,
        randomFleet: null,
        connected: false
    });

    return {
        subscribe,
        setGame: (game: GameSnapshot) => update((s) => ({ ...s, game })),
        setPlayer: (player: Player) => update((s) => ({ ...s, player })),
        setRandomFleet: (fleet: ShipPlacement[]) => update((s) => ({ ...s, randomFleet: fleet })),
        setConnected: (val: boolean) => update((s) => ({ ...s, connected: val })),
        reset: () => set({ game: null, player: null, randomFleet: null, connected: false })
    };
}

export const gameStore = createGameStore();
