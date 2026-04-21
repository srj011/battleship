import { apiHealth } from '$lib/stores/app';
import type { ApiErrorCode } from '$lib/types';

const API_BASE = 'http://localhost:3000/api';
const API_V1 = API_BASE + '/v1';

export class ApiError extends Error {
    code: ApiErrorCode;
    message: string;

    constructor(code: ApiErrorCode, message: string) {
        super(message);
        this.code = code;
        this.message = message;
    }
}

export async function checkHealth() {
    apiHealth.update((state) => {
        if (state !== 'online') return 'checking';
        return state;
    });

    try {
        const res = await fetch(`${API_BASE}/health`);
        apiHealth.set(res.ok ? 'online' : 'offline');
    } catch {
        apiHealth.set('offline');
    }
}

async function apiFetch(path: string, options?: RequestInit) {
    try {
        const res = await fetch(`${API_V1}${path}`, options);
        if (!res.ok) {
            let data = null;

            try {
                data = await res.json();
            } catch {
                const text = await res.text().catch(() => null);

                throw new ApiError('internal_error', text || 'Unexpected server error');
            }

            if (data?.code && data?.message) {
                throw new ApiError(data.code, data.message);
            }

            throw new ApiError('internal_error', data?.message || 'Unexpected server error');
        }
        return res.json();
    } catch (err) {
        if (err instanceof ApiError) throw err;

        throw new ApiError(
            'internal_error',
            'Unable to reach the server. Check your internet connection.'
        );
    }
}

export async function createGame(mode: 'ai' | 'multiplayer') {
    return apiFetch('/game', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mode })
    });
}

export async function joinGame(code: string) {
    return apiFetch(`/game/${code}/join`, {
        method: 'POST'
    });
}

export async function getGame(code: string, player_token: string) {
    return apiFetch(`/game/${code}?player_token=${player_token}`);
}

export async function verifySession(
    code: string,
    player_token: string
): Promise<'valid' | 'not-found' | 'error'> {
    try {
        await getGame(code, player_token);
        return 'valid';
    } catch (err) {
        if (err instanceof ApiError) {
            if (err.code === 'session_not_found') {
                return 'not-found';
            }
        }
        return 'error';
    }
}
