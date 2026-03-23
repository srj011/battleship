import type { GameSnapshot, ShipPlacement } from "$lib/types";
import { writable } from "svelte/store";

type GameStore = {
    game: GameSnapshot | null;
    previewFleet: ShipPlacement[] | null;
    connected: boolean;
};

function createGameStore() {
    const { subscribe, update, set } = writable<GameStore>({
        game: null,
        previewFleet: null,
        connected: false,
    });

    return {
        subscribe,
        setGame: (game: GameSnapshot) => update((s) => ({ ...s, game })),
        setPreviewFleet: (fleet: ShipPlacement[]) => update((s) => ({ ...s, previewFleet: fleet })),
        setConnected: (val: boolean) => update((s) => ({ ...s, connected: val })),
        reset: () => set({ game: null, previewFleet: null, connected: false })
    };
}

export const gameStore = createGameStore();
