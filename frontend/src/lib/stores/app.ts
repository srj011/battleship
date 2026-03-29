import { writable } from 'svelte/store';

export type ApiHealthStatus = 'checking' | 'online' | 'offline';

export const apiHealth = writable<ApiHealthStatus>('checking');
