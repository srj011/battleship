<script lang="ts">
    import * as AlertDialog from '$lib/components/ui/alert-dialog';
    import { Button } from '$lib/components/ui/button';
    import { gameStore } from '$lib/stores/game';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';
    import { leaveGame } from '$lib/api/websocket';

    function handleLeave() {
        leaveGame();
        gameStore.reset();
        goto(resolve('/'));
    }
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <Button variant="destructive">Leave Game</Button>
    </AlertDialog.Trigger>

    <AlertDialog.Content size="sm">
        <AlertDialog.Header>
            <AlertDialog.Title>Leave Game?</AlertDialog.Title>
            <AlertDialog.Description>
                You will leave the match and may forfeit the game.
            </AlertDialog.Description>
        </AlertDialog.Header>

        <AlertDialog.Footer>
            <AlertDialog.Action variant="destructive" onclick={handleLeave}
                >Leave</AlertDialog.Action
            >
            <AlertDialog.Cancel variant="outline">Cancel</AlertDialog.Cancel>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
