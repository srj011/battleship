import { writable } from 'svelte/store';
import type { Notification } from '$lib/types';

function createNotificationStore() {
    const { subscribe, update } = writable<Notification[]>([]);

    function push(notification: Omit<Notification, 'id'>, duration: number = 3000) {
        const id = crypto.randomUUID();
        update((n) => [...n, { ...notification, id }]);

        if (duration > 0) {
            setTimeout(() => {
                remove(id);
            }, duration);
        }

        return id;
    }

    function remove(id: string) {
        update((n) => n.filter((item) => item.id !== id));
    }

    return {
        subscribe,
        push,
        remove
    };
}

export const notificationStore = createNotificationStore();
