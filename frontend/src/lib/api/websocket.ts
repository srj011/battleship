/* eslint-disable no-console */
import { gameStore } from '$lib/stores/game';
import { notificationStore } from '$lib/stores/notification';
import { get } from 'svelte/store';
import type { ClientMessage, ServerMessage } from '$lib/types';
import { verifySession } from '$lib/api/client';

let socket: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

let currentCode: string | null = null;
let currentToken: string | null = null;

const MAX_RECONNECT_ATTEMPTS = 10;

function getReconnectDelay(attempt: number): number {
    const base = Math.min(30_000, Math.pow(2, attempt) * 1000);
    const jitter = Math.random() * 1000;
    return base + jitter;
}

export function connectWS(code: string, token: string) {
    currentCode = code;
    currentToken = token;

    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }

    if (socket) {
        if (socket.readyState !== WebSocket.CLOSED) {
            console.warn('[WS] Already connecting/connected');
            return;
        }
        socket.close();
        socket = null;
    }

    const url = `ws://localhost:3000/api/v1/game/${code}/ws?player_token=${token}`;
    socket = new WebSocket(url);

    if (get(gameStore).connection.state !== 'reconnecting') {
        gameStore.dispatch({ type: 'CONNECT' });
    }

    socket.onopen = () => {
        console.log('[WS] Connected');

        gameStore.dispatch({ type: 'CONNECTED' });
    };

    socket.onclose = (event) => {
        console.log('[WS] Disconnected', event.code);

        socket = null;
        const connection = get(gameStore).connection;

        if (
            connection.state === 'idle' ||
            connection.state === 'unreachable' ||
            connection.state === 'invalid-session' ||
            event.code === 1000
        ) {
            return;
        }

        if (connection.state === 'connecting') {
            verifySession(code, token).then((status) => {
                console.log('verifySession output: ', status);

                if (get(gameStore).connection.state !== 'connecting') return;

                if (status === 'not-found') {
                    gameStore.dispatch({ type: 'INVALID_SESSION' });

                    if (reconnectTimer) {
                        clearTimeout(reconnectTimer);
                        reconnectTimer = null;
                    }

                    return;
                }

                gameStore.dispatch({ type: 'DISCONNECTED' });
                scheduleReconnect();
            });
        } else {
            gameStore.dispatch({ type: 'DISCONNECTED' });
            scheduleReconnect();
        }
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

            case 'player_disconnected':
                gameStore.setDisconnect(msg.info);
                break;

            case 'player_reconnected':
                gameStore.clearDisconnect();
                break;

            case 'rematch_cancelled':
                notificationStore.push({
                    title: 'Rematch cancelled',
                    message: `${msg.player === get(gameStore).game?.player ? 'You' : 'Opponent'} cancelled the rematch`,
                    type: 'info'
                });
                break;

            case 'rematch_rejected':
                notificationStore.push({
                    title: 'Rematch declined',
                    message: `${msg.player === get(gameStore).game?.player ? 'You' : 'Opponent'} declined the rematch request`,
                    type: 'info'
                });
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

    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }

    let connection = get(gameStore).connection;

    if (connection.state !== 'reconnecting') {
        return;
    }

    if (connection.attempt >= MAX_RECONNECT_ATTEMPTS) {
        console.error('[WS] Max reconnect attempts reached');
        gameStore.dispatch({ type: 'MAX_RETRIES' });
        return;
    }

    gameStore.dispatch({ type: 'RETRY' });

    connection = get(gameStore).connection;
    const delay = getReconnectDelay(connection.attempt);

    console.log(`[WS] Reconnecting in ${Math.round(delay)}ms (attempt ${connection.attempt})`);

    reconnectTimer = setTimeout(() => {
        connection = get(gameStore).connection;
        if (connection.state !== 'reconnecting') {
            return;
        }

        if (!currentCode || !currentToken) {
            return;
        }

        console.log(`[WS] Reconnecting... (attempt ${connection.attempt})`);

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
    socket?.close(1000, 'Manual disconnect');
    socket = null;

    if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
    }
}

export function leaveGame() {
    if (socket && socket.readyState === WebSocket.OPEN) {
        const msg: ClientMessage = { type: 'leave_game' };
        sendWS(msg);

        const closeAfterFlush = () => {
            if (!socket) return;

            if (socket.bufferedAmount === 0) {
                gameStore.dispatch({ type: 'LEAVE' });
                disconnectWS();
            } else {
                setTimeout(closeAfterFlush, 10);
            }
        };
        closeAfterFlush();
    } else {
        disconnectWS();
    }
}

if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', () => {
        if (
            document.visibilityState === 'visible' &&
            (!socket || socket.readyState === WebSocket.CLOSED) &&
            currentCode &&
            currentToken &&
            get(gameStore).connection.state === 'reconnecting'
        ) {
            console.log('[WS] Tab active -> reconnecting');
            connectWS(currentCode, currentToken);
        }
    });
}
