<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { hoverLift } from "$lib/motion";
  import type { HeroMeta } from "$lib/types";
  import { pct } from "$lib/format";
  import { store } from "$lib/store.svelte";

  let { hero, rank }: { hero: HeroMeta; rank: number } = $props();

  let broken = $state(false);

  // No portrait dir known yet (or backend not wired) -> fall back straight
  // to initials instead of attempting to load a non-existent image.
  const src = $derived(
    store.portraitDir ? convertFileSrc(`${store.portraitDir}/${hero.slug}.png`) : null
  );
  const initials = $derived(
    hero.name
      .split(/\s+/)
      .filter(Boolean)
      .map((w) => w[0])
      .join("")
      .slice(0, 2)
      .toUpperCase()
  );
</script>

<div
  use:hoverLift
  role="group"
  class="flex items-center gap-2 rounded-lg border border-border bg-card/40 p-1.5"
>
  <div class="relative size-9 shrink-0 overflow-hidden rounded-md bg-muted">
    {#if src && !broken}
      <img
        {src}
        alt={hero.name}
        class="size-full object-cover"
        onerror={() => (broken = true)}
      />
    {:else}
      <div
        class="grid size-full place-items-center text-[10px] font-semibold text-muted-foreground"
      >
        {initials}
      </div>
    {/if}
    <span
      class="absolute bottom-0 left-0 rounded-tr bg-background/80 px-1 text-[9px] leading-tight text-muted-foreground"
    >
      {rank}
    </span>
  </div>
  <div class="min-w-0 flex-1">
    <div class="truncate text-xs font-medium">{hero.name}</div>
    <div class="flex items-center gap-1.5 text-[10px] text-muted-foreground">
      <span class="font-semibold text-foreground">{pct(hero.winrate)}</span>
      <span>{pct(hero.pickrate)}</span>
    </div>
  </div>
</div>
