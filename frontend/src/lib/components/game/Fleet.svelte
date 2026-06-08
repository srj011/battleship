<script lang="ts">
    import { SHIP_LENGTHS } from '$lib/game/config';
    import type { FleetView, ShipStatus } from '$lib/types';

    interface Props {
        fleet: FleetView;
        variant: 'player' | 'opponent';
    }

    const { fleet, variant }: Props = $props();

    function getCellColor(ship: ShipStatus, i: number, size: number) {
        if (ship.sunk) return 'bg-destructive/40';
        if (variant === 'player') {
            if (ship.damage && i >= size - ship.damage) return 'bg-board-hit';
            return 'bg-primary/60';
        }
        return 'bg-primary/60';
    }
</script>

<div class="grid w-full grid-cols-2 gap-4">
    {#if fleet}
        {#each fleet.ships as ship (ship.ship_type)}
            <div
                class="flex flex-col gap-2 rounded-md border border-border bg-card/40 px-4 py-1.5 backdrop-blur-md"
            >
                <div
                    class={`absolute top-0 bottom-0 left-0 w-1 rounded-l-lg
                    ${ship.sunk ? 'bg-destructive/60' : 'bg-primary/60'}`}
                ></div>
                <!-- Ship header -->
                <div class="flex justify-between text-xs font-medium tracking-wide">
                    <span
                        class={`uppercase ${ship.sunk ? 'text-destructive/80' : 'text-primary/90'}`}
                        >{ship.ship_type}</span
                    >
                    {#if ship.sunk}
                        <span class="text-destructive">SUNK</span>
                    {:else if variant === 'opponent'}
                        <span class="text-primary/70">ACTIVE</span>
                    {:else if ship.damage === 0}
                        <span class="text-primary/70">ACTIVE</span>
                    {:else}
                        <span class="text-primary/90">
                            {Math.round(
                                100 - ((ship.damage ?? 0) / SHIP_LENGTHS[ship.ship_type]) * 100
                            )}%
                        </span>
                    {/if}
                </div>

                <!-- Ship bar -->
                <div class="flex w-full gap-1">
                    {#each Array(SHIP_LENGTHS[ship.ship_type]) as _, i (i)}
                        <div
                            class={`h-0.75 flex-1 ${getCellColor(ship, i, SHIP_LENGTHS[ship.ship_type])}`}
                        ></div>
                    {/each}
                </div>
            </div>
        {/each}
    {/if}
</div>
