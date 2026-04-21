<script lang="ts">
    import { notificationStore } from '$lib/stores/notification';
    import Notification from './Notification.svelte';
    import { flip } from 'svelte/animate';
    import { cubicOut } from 'svelte/easing';

    const notifications = $derived($notificationStore);

    function remove(id: string) {
        notificationStore.remove(id);
    }
</script>

<div class="fixed top-4 right-4 z-50 flex flex-col gap-3">
    {#each notifications as notification (notification.id)}
        <div animate:flip={{ duration: 150, easing: cubicOut }}>
            <Notification {notification} onClose={remove} />
        </div>
    {/each}
</div>
