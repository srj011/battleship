import { gameStore } from '$lib/stores/game';
import type { ClientMessage, ServerMessage } from '$lib/types';

let socket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

let currentCode: string | null = null;
let currentToken: string | null = null;

let reconnectAttempts = 0;
let manuallyDisconnected = false;

const MAX_RECONNECT_ATTEMPTS = 10;

function getReconnectDelay(attempt: number): number {
    const base = Math.min(30_000, Math.pow(2, attempt) * 1000);
    const jitter = Math.random() * 1000;
    return base + jitter;
}

export function connectWS(code: string, token: string) {
    currentCode = code;
    currentToken = token;
    manuallyDisconnected = false;

    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }

    if (socket) {
        socket.close();
    }

    const url = `ws://localhost:3000/api/v1/game/${code}/ws?player_token=${token}`;
    socket = new WebSocket(url);

    socket.onopen = () => {
        console.log('[WS] Connected');

        reconnectAttempts = 0;
        gameStore.setConnected(true);
        gameStore.setReconnecting(false);
        gameStore.resetReconnectAttempts();
    };

    socket.onclose = (event) => {
        console.log('[WS] Disconnected', event.code);

        gameStore.setConnected(false);
        socket = null;

        if (manuallyDisconnected || event.code === 1000) {
            return;
        }

        scheduleReconnect();
    };

    socket.onmessage = (messageEvent) => {
        const msg: ServerMessage = JSON.parse(messageEvent.data);
        console.log(msg);

        switch (msg.type) {
            case 'game_state':
                gameStore.setGame(msg);
                break;

            case 'game_update':
                gameStore.applyGameUpdate(msg);
                console.log(msg.event);
                break;

            case 'random_fleet':
                gameStore.setRandomFleet(msg.fleet);
                break;

            case 'error':
                console.error('[WS] Error: ', msg.message);
                break;
        }
    };

    socket.onerror = (err) => {
        console.error('[WS] Error: ', err);
        socket?.close();
    };
}

function scheduleReconnect() {
    if (!currentCode || !currentToken) return;

    if (reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
        console.error('[WS] Max reconnect attempts reached');
        gameStore.setReconnecting(false);
        return;
    }

    const delay = getReconnectDelay(reconnectAttempts);

    gameStore.setReconnecting(true);

    console.log(`[WS] Reconnecting in ${Math.round(delay)}ms (attempt ${reconnectAttempts + 1})`);

    reconnectTimer = setTimeout(() => {
        reconnectAttempts++;
        gameStore.incrementReconnectAttempts();
        connectWS(currentCode!, currentToken!);
    }, delay);
}

export function sendWS(message: ClientMessage) {
    if (!socket || socket.readyState !== WebSocket.OPEN) {
        console.warn('[WS] Cannot send, socket not open');
        return;
    }

    socket.send(JSON.stringify(message));
}

export function disconnectWS() {
    manuallyDisconnected = true;

    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }

    socket?.close(1000, 'Manual disconnect');
    socket = null;

    currentCode = null;
    currentToken = null;
}

if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
        if (
            document.visibilityState === 'visible' &&
            !socket &&
            currentCode &&
            currentToken &&
            !manuallyDisconnected
        ) {
            console.log('[WS] Tab active -> reconnecting');
            connectWS(currentCode, currentToken);
        }
    });
}
