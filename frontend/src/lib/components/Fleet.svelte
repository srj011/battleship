<script lang="ts">
    import { SHIP_LENGTHS } from '$lib/game/config';
    import type { FleetView, ShipStatus } from '$lib/types';

    interface Props {
        fleet: FleetView;
        variant: 'player' | 'opponent';
    }

    const { fleet, variant }: Props = $props();

    function getCellColor(ship: ShipStatus, i: number) {
        if (variant === 'player') {
            if (ship.sunk) return 'bg-red-500';
            if (ship.damage && i < ship.damage) return 'bg-gray-800';
            return 'bg-indigo-500';
        }
        return ship.sunk ? 'bg-green-500' : 'bg-gray-300';
    }
</script>

<div class="space-y-1">
    <h2 class="text-sm font-semibold">
        {variant === 'player' ? 'Your' : "Opponent's"} Fleet
    </h2>

    <div class="flex flex-col gap-4">
        {#if fleet}
            {#each fleet.ships as ship (ship.ship_type)}
                <div class="flex items-center gap-2">
                    <!-- Ship bar -->
                    <div class="flex gap-0.5">
                        {#each Array(SHIP_LENGTHS[ship.ship_type]) as _, i (i)}
                            <div class={`h-5 w-5 rounded-sm ${getCellColor(ship, i)}`}></div>
                        {/each}
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>
