import { PUBLIC_API_URL } from '$env/static/public';

if (!PUBLIC_API_URL) {
    throw new Error('PUBLIC_API_URL is not configured');
}

export const API_URL = PUBLIC_API_URL;
export const API_BASE = `${API_URL}/api`;
export const API_V1 = `${API_BASE}/v1`;

export function getWsUrl(path: string): string {
    return `${API_URL.replace(/^http/, 'ws')}${path}`;
}
