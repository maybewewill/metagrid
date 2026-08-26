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
  class="flex items-center gap-2.5 rounded-sm px-2 py-2 transition-colors hover:bg-accent"
>
  <span class="w-3.5 shrink-0 text-right font-mono text-[12px] font-medium tabular-nums text-muted-foreground/60">
    {rank}
  </span>

  <div
    class="relative h-[50px] w-[36px] shrink-0 overflow-hidden rounded-sm bg-muted ring-1 ring-border shadow-sm"
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

  <div class="min-w-0 flex-1">
    <div class="truncate text-[12.5px] font-semibold leading-tight text-zinc-100">{hero.name}</div>
    <div class="mt-0.5 flex items-center gap-1 text-[11px] leading-tight text-muted-foreground">
      <span>{pct(hero.pickrate)} pick</span>
      {#if hero.d2pt_rating && hero.d2pt_rating > 0}
        <span class="text-white/20">•</span>
        <span class="font-mono text-zinc-300 font-medium" title="D2PT Rating">{hero.d2pt_rating}</span>
      {/if}
    </div>
  </div>

  <span class="shrink-0 text-[12.5px] font-bold tabular-nums" style:color={wrColor}>
    {pct(hero.winrate)}
  </span>
</div>
