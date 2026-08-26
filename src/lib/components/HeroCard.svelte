<script lang="ts">
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";

  let { hero, rank }: { hero: HeroMeta; rank: number } = $props();

  let broken = $state(false);

  // Tall vertical portraits — the same art Dota's in-client hero grid uses.
  const src = $derived(
    `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/heroes/${hero.slug}_vert.jpg`,
  );
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
  class="flex items-center gap-2.5 rounded-md px-1.5 py-1 transition-colors hover:bg-accent"
>
  <span class="w-3 shrink-0 text-right font-mono text-[11px] tabular-nums text-muted-foreground/60">
    {rank}
  </span>

  <div
    class="relative h-11 w-9 shrink-0 overflow-hidden rounded-[5px] bg-muted ring-1 ring-border"
  >
    {#if !broken}
      <img
        {src}
        alt={hero.name}
        loading="lazy"
        class="size-full object-cover object-top"
        onerror={() => (broken = true)}
      />
    {:else}
      <div class="grid size-full place-items-center text-[10px] font-semibold text-muted-foreground">
        {initials}
      </div>
    {/if}
  </div>

  <div class="min-w-0 flex-1">
    <div class="truncate text-[13px] font-medium leading-tight">{hero.name}</div>
    <div class="text-[11px] leading-tight text-muted-foreground">{pct(hero.pickrate)} pick</div>
  </div>

  <span class="shrink-0 text-[13px] font-semibold tabular-nums" style:color={wrColor}>
    {pct(hero.winrate)}
  </span>
</div>
