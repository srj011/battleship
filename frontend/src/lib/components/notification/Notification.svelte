<script lang="ts">
    import * as Alert from '$lib/components/ui/alert';
    import Icon from '@iconify/svelte';
    import type { Notification, NotificationType } from '$lib/types';
    import { fade, fly } from 'svelte/transition';

    interface Props {
        notification: Notification;
        onClose: (id: string) => void;
    }
    const { notification, onClose }: Props = $props();

    const type = $derived(notification.type ?? 'info');

    function getVariant(type: NotificationType) {
        return type === 'error' ? 'destructive' : 'default';
    }

    function getIcon(type: NotificationType) {
        switch (type) {
            case 'success':
                return 'material-symbols:check-circle-rounded';
            case 'info':
                return 'mdi:info-circle';
            default:
                return 'mdi:alert-circle';
        }
    }

    const variant = $derived(getVariant(type));
    const icon = $derived(getIcon(type));
</script>

<div in:fly={{ y: -32, duration: 300 }} out:fade={{ duration: 500 }}>
    <Alert.Root {variant} class="relative">
        <Icon {icon} width="4" height="4" />
        <Alert.Title>{notification.title}</Alert.Title>
        <Alert.Description>{notification.message}</Alert.Description>
        <button
            class="absolute top-2 right-2 text-sm opacity-70 hover:opacity-100"
            onclick={() => onClose(notification.id)}
        >
            ✕
        </button>
    </Alert.Root>
</div>
