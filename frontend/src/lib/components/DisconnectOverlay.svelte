<script lang="ts">
    import { fade } from 'svelte/transition';

    const { seconds } = $props();
</script>

<div
    class="absolute inset-0 z-50 flex items-center justify-center bg-slate-950/60 backdrop-blur-sm"
    transition:fade
>
    <div
        class="rounded-xl border border-cyan-400/30 bg-slate-900/90 px-6 py-4 text-center shadow-lg"
    >
        <p class="mb-1 font-semibold text-slate-200">Opponent Disconnected</p>

        <p class="text-sm text-slate-400">
            Game will end in
            <span class="font-semibold text-cyan-400">{seconds}s</span>
            if they dont return
        </p>
    </div>
</div>
