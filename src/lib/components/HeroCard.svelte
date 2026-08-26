<script lang="ts">
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";

  let { hero, rank }: { hero: HeroMeta; rank: number } = $props();

  let broken = $state(false);

  let currentSrc = $state("");
  
  $effect(() => {
    currentSrc = `https://dota2protracker.com/static/heroes/${hero.slug}_vert.jpg`;
    broken = false;
  });

  function handleError() {
    if (currentSrc.includes("dota2protracker.com")) {
      currentSrc = `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/heroes/${hero.slug}_vert.jpg`;
    } else {
      broken = true;
    }
  }

  const initials = $derived(
    hero.name
      .split(/\s+/)
      .filter(Boolean)
      .map((w) => w[0])
      .join("")
      .slice(0, 2)
      .toUpperCase(),
  );

  // Colour the win-rate by value: red (~48%) → amber (~52%) → green (~56%+).
  const wrColor = $derived.by(() => {
    const t = Math.max(0, Math.min(1, (hero.winrate - 0.48) / 0.08));
    const hue = 20 + t * 130;
    return `oklch(0.82 0.15 ${hue})`;
  });
</script>

<div
  use:hoverLift
  role="group"
  class="flex items-center gap-2 rounded-sm border border-white/[0.04] bg-zinc-900/40 px-2 py-1.5 transition-all duration-150 hover:border-white/[0.12] hover:bg-zinc-800/80"
>
  <span class="w-3.5 shrink-0 text-center font-mono text-[11px] font-semibold tabular-nums text-zinc-500">
    {rank}
  </span>

  <div
    class="relative h-[44px] w-[32px] shrink-0 overflow-hidden rounded-sm bg-zinc-900 ring-1 ring-white/10 shadow-sm"
  >
    {#if !broken}
      <img
        src={currentSrc}
        alt={hero.name}
        loading="lazy"
        class="size-full object-cover object-top"
        onerror={handleError}
      />
    {:else}
      <div class="grid size-full place-items-center text-[10px] font-semibold text-muted-foreground">
        {initials}
      </div>
    {/if}
  </div>

  <div class="min-w-0 flex-1 flex flex-col justify-center gap-0.5">
    <div class="truncate text-[12.5px] font-bold leading-tight text-zinc-100">{hero.name}</div>
    <div class="flex items-center gap-1.5 text-[10.5px] font-mono leading-none text-zinc-400 whitespace-nowrap overflow-hidden">
      <span>{pct(hero.pickrate)}</span>
      {#if hero.d2pt_rating && hero.d2pt_rating > 0}
        <span class="text-zinc-600">·</span>
        <span class="font-medium text-zinc-300" title="D2PT Rating">{hero.d2pt_rating}</span>
      {/if}
    </div>
  </div>

  <span
    class="shrink-0 rounded px-1.5 py-0.5 font-mono text-[11.5px] font-bold tabular-nums"
    style:color={wrColor}
    style:background="rgba(255, 255, 255, 0.04)"
  >
    {pct(hero.winrate)}
  </span>
</div>
