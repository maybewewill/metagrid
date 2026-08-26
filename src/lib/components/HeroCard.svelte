<script lang="ts">
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";

  let { hero }: { hero: HeroMeta; rank?: number } = $props();

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
  class="group flex h-[62px] items-center overflow-hidden rounded-sm border border-zinc-800/60 bg-zinc-900/40 transition-all duration-150 hover:border-zinc-700 hover:bg-zinc-800/80"
>
  <!-- Hero Portrait flush to the left -->
  <div
    class="relative h-full w-[46px] shrink-0 overflow-hidden bg-zinc-950 border-r border-zinc-800/80"
  >
    {#if !broken}
      <img
        src={currentSrc}
        alt={hero.name}
        loading="lazy"
        class="size-full object-cover object-top transition-transform duration-200 group-hover:scale-105"
        onerror={handleError}
      />
    {:else}
      <div class="grid size-full place-items-center text-[10px] font-semibold text-muted-foreground">
        {initials}
      </div>
    {/if}
  </div>

  <!-- Hero Details -->
  <div class="flex min-w-0 flex-1 flex-col justify-center px-2.5 py-1 gap-0.5">
    <div class="truncate text-[12.5px] font-extrabold uppercase tracking-wide text-zinc-100 group-hover:text-white">
      {hero.name}
    </div>
    <div class="flex items-center gap-1.5 text-[11px] font-mono leading-none text-zinc-400 whitespace-nowrap overflow-hidden">
      <span>{pct(hero.pickrate)} Pick</span>
      {#if hero.d2pt_rating && hero.d2pt_rating > 0}
        <span class="text-zinc-600">·</span>
        <span class="font-medium text-zinc-300" title="D2PT Rating">{hero.d2pt_rating}</span>
      {/if}
    </div>
  </div>

  <!-- Winrate -->
  <div class="shrink-0 pr-2.5 flex flex-col items-end justify-center">
    <span
      class="font-mono text-[13px] font-black tracking-tight tabular-nums"
      style:color={wrColor}
    >
      {pct(hero.winrate)}
    </span>
  </div>
</div>
