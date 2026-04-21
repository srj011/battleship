import { apiHealth } from '$lib/stores/app';

const API_BASE = 'http://localhost:3000/api';
const API_V1 = API_BASE + '/v1';

export async function checkHealth() {
    apiHealth.set('checking');

    try {
        const res = await fetch(`${API_BASE}/health`);
        apiHealth.set(res.ok ? 'online' : 'offline');
    } catch {
        apiHealth.set('offline');
    }
}

async function apiFetch(path: string, options?: RequestInit) {
    const res = await fetch(`${API_V1}${path}`, options);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json();
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

export async function verifySession(
    code: string,
    token: string
): Promise<'valid' | 'not-found' | 'error'> {
    try {
        const res = await fetch(`${API_V1}/game/${code}?player_token=${token}`);

        if (res.status === 404) {
            return 'not-found';
        }
        if (res.ok) return 'valid';
        return 'error';
    } catch {
        return 'error';
    }
}
