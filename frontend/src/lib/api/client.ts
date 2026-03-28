const BASE_URL = 'http://localhost:3000/api/v1';

export async function createGame(mode: 'ai' | 'multiplayer') {
    const res = await fetch(`${BASE_URL}/game`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mode })
    });

    return res.json();
}

export async function joinGame(code: string) {
    const res = await fetch(`${BASE_URL}/game/${code}/join`, {
        method: 'POST'
    });

    return res.json();
}
