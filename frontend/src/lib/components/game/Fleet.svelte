<script lang="ts">
    import { SHIP_LENGTHS } from '$lib/game/config';
    import type { FleetView, ShipStatus } from '$lib/types';

    interface Props {
        fleet: FleetView;
        variant: 'player' | 'opponent';
    }

    const { fleet, variant }: Props = $props();

    function getCellColor(ship: ShipStatus, i: number, size: number) {
        if (ship.sunk) return 'bg-red-400';
        if (variant === 'player') {
            if (ship.damage && i >= size - ship.damage) return 'bg-gray-300';
            return 'bg-indigo-500';
        }
        return 'bg-gray-300';
    }
</script>

<div class="grid w-full grid-cols-2 gap-6">
    {#if fleet}
        {#each fleet.ships as ship (ship.ship_type)}
            <div
                class={`flex flex-col gap-2 border-l-2 px-2 py-1 ${
                    ship.sunk ? 'border-red-500 opacity-50' : 'border-indigo-500'
                }`}
            >
                <!-- Ship header -->
                <div class="flex justify-between text-xs tracking-wide">
                    <span class={`uppercase ${ship.sunk ? 'line-through' : ''}`}
                        >{ship.ship_type}</span
                    >
                    {#if ship.sunk}
                        <span class="text-red-400">SUNK</span>
                    {:else}
                        <span>SIZE: {SHIP_LENGTHS[ship.ship_type]}</span>
                    {/if}
                </div>

                <!-- Ship bar -->
                <div class="flex w-full gap-1">
                    {#each Array(SHIP_LENGTHS[ship.ship_type]) as _, i (i)}
                        <div
                            class={`h-1 flex-1 ${getCellColor(ship, i, SHIP_LENGTHS[ship.ship_type])}`}
                        ></div>
                    {/each}
                </div>
            </div>
        {/each}
    {/if}
</div>
