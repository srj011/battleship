<script lang="ts">
    import * as AlertDialog from '$lib/components/ui/alert-dialog';

    import { gameStore } from '$lib/stores/game';
    import { sendWS } from '$lib/api/websocket';
    import type { ClientMessage } from '$lib/types';

    const player = $derived($gameStore.game?.player);
    const rematch = $derived($gameStore.game?.rematch_state);
    const rematchSelf = $derived(rematch?.type === 'requested' && rematch.by === player);
    const rematchOpponent = $derived(rematch?.type === 'requested' && rematch.by !== player);

    const showWaiting = $derived(rematchSelf && !rematchOpponent);
    const showIncoming = $derived(!rematchSelf && rematchOpponent);

    function accept() {
        const msg: ClientMessage = { type: 'request_rematch' };
        sendWS(msg);
    }

    function cancel() {
        const msg: ClientMessage = { type: 'cancel_rematch' };
        sendWS(msg);
    }

    function reject() {
        const msg: ClientMessage = { type: 'reject_rematch' };
        sendWS(msg);
        return;
    }
</script>

<AlertDialog.Root open={showWaiting}>
    <AlertDialog.Content size="sm">
        <AlertDialog.Header>
            <AlertDialog.Title>
                Waiting for opponent
                <span class="dots inline-block w-[1.5ch] text-left"></span>
            </AlertDialog.Title>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel onclick={cancel}>Cancel</AlertDialog.Cancel>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>

<AlertDialog.Root open={showIncoming}>
    <AlertDialog.Content size="sm">
        <AlertDialog.Header>
            <AlertDialog.Title>Opponent wants a rematch.</AlertDialog.Title>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Action onclick={accept}>Accept</AlertDialog.Action>
            <AlertDialog.Cancel onclick={reject}>Decline</AlertDialog.Cancel>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>

<style>
    .dots::after {
        content: '';
        animation:
            dots 1.8s steps(4, end) infinite,
            float 1.8s ease-in-out infinite;
    }

    @keyframes dots {
        0% {
            content: '';
        }
        25% {
            content: '.';
        }
        50% {
            content: '..';
        }
        75% {
            content: '...';
        }
        100% {
            content: '';
        }
    }
    @keyframes float {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-2px);
        }
    }
</style>
