<script lang="ts">
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";
  import { getHeroPortraitUrl } from "$lib/utils";

  let { hero }: { hero: HeroMeta; rank?: number } = $props();

  let broken = $state(false);
  let currentSrc = $state("");

  $effect(() => {
    currentSrc = getHeroPortraitUrl(hero.slug);
    broken = false;
  });

  function handleError() {
    broken = true;
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

  const wrColor = $derived.by(() => {
    const t = Math.max(0, Math.min(1, (hero.winrate - 0.48) / 0.08));
    const hue = 20 + t * 130;
    return `oklch(0.82 0.15 ${hue})`;
  });
</script>

<div
  use:hoverLift
  role="group"
  class="group flex flex-1 min-h-0 items-center overflow-hidden rounded-sm border border-zinc-800/60 bg-zinc-900/40 transition-all duration-150 hover:border-zinc-700 hover:bg-zinc-800/80"
>
  <div
    class="relative h-full w-[52px] shrink-0 overflow-hidden bg-zinc-950 border-r border-zinc-800/80"
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
      <div class="grid size-full place-items-center text-xs font-semibold text-muted-foreground">
        {initials}
      </div>
    {/if}
  </div>

  <div class="flex min-w-0 flex-1 flex-col justify-center px-3.5 py-1 gap-1">
    <div class="truncate text-[13.5px] font-extrabold uppercase tracking-wide text-zinc-100 group-hover:text-white">
      {hero.name}
    </div>
    <div class="flex items-center gap-2 text-[12px] font-mono leading-none text-zinc-400 whitespace-nowrap overflow-hidden">
      <span>{pct(hero.pickrate)} Pick</span>
      {#if hero.d2pt_rating && hero.d2pt_rating > 0}
        <span class="text-zinc-600">·</span>
        <span class="font-bold text-purple-400" title="D2PT Rating">{hero.d2pt_rating}</span>
      {/if}
    </div>
  </div>

  <div class="shrink-0 pr-3.5 flex flex-col items-end justify-center">
    <span
      class="font-mono text-[14px] font-black tracking-tight tabular-nums"
      style:color={wrColor}
    >
      {pct(hero.winrate)}
    </span>
  </div>
</div>
