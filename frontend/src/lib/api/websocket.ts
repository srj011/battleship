import { gameStore } from "$lib/stores/game";
import type { ServerMessage } from "$lib/types";

let socket: WebSocket | null = null;

export function connectWS(code: string, token: string) {
    socket = new WebSocket(`ws://localhost:3000/api/v1/game/${code}/ws?player_token=${token}`);

    socket.onopen = () => {
        gameStore.setConnected(true);
    };

    socket.onclose = () => {
        gameStore.setConnected(false);
    };

    socket.onmessage = (messageEvent) => {
        const msg: ServerMessage = JSON.parse(messageEvent.data);
        console.log(msg);

        switch (msg.type) {
            case "game_state":
                gameStore.setGame(msg);
                gameStore.setPlayer(msg.player);
                break;

            case "game_update":
                gameStore.setGame({
                    turn: msg.turn,
                    status: msg.status,
                    player_board: msg.player_board,
                    opponent_board: msg.opponent_board,
                });

                console.log(msg.event);
                break;

            case 'random_fleet':
                gameStore.setRandomFleet(msg.fleet);
                break;

            case "error":
                console.error(msg.message);
                break;
        }
    };
}

export function sendWS(message: any) {
    socket?.send(JSON.stringify(message));
}

export function disconnectWS() {
    socket?.close();
    socket = null;
}
