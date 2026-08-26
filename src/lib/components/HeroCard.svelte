<script lang="ts">
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";

  let { hero, rank }: { hero: HeroMeta; rank: number } = $props();

  let broken = $state(false);

  // Hero portraits come straight from Valve's CDN by internal slug (the same
  // `<slug>.png` naming Dota's own client uses); `img-src` in the CSP allows
  // this host. Falls back to initials if the image can't load (offline).
  const src = $derived(
    `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/dota_react/heroes/${hero.slug}.png`,
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
</script>

<div
  use:hoverLift
  role="group"
  class="flex items-center gap-2.5 rounded-md px-1.5 py-1 transition-colors hover:bg-muted"
>
  <span class="w-3 shrink-0 text-right font-mono text-[11px] tabular-nums text-muted-foreground/70">
    {rank}
  </span>

  <div
    class="relative h-7 w-[52px] shrink-0 overflow-hidden rounded-[5px] bg-muted ring-1 ring-border"
  >
    {#if !broken}
      <img
        {src}
        alt={hero.name}
        loading="lazy"
        class="size-full object-cover"
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
    <div class="text-[11px] leading-tight text-muted-foreground">
      {pct(hero.pickrate)}
      {" "}pick
    </div>
  </div>

  <span class="shrink-0 text-[13px] font-semibold tabular-nums">{pct(hero.winrate)}</span>
</div>
