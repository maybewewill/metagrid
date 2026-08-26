<script lang="ts">
  import { _ } from "svelte-i18n";
  import HeroCard from "$lib/components/HeroCard.svelte";
  import { pct, roleLabel } from "$lib/format";
  import { prefersReducedMotion } from "$lib/motion";
  import { animate } from "motion";
  import type { RoleMeta } from "$lib/types";

  let { role }: { role: RoleMeta } = $props();

  const n = $derived(Number(role.position.slice(3)));
  let listEl = $state<HTMLDivElement | null>(null);

  // Light mount stagger for the hero rows — skipped under reduced-motion.
  $effect(() => {
    if (!listEl || prefersReducedMotion()) return;
    const items = Array.from(listEl.children) as HTMLElement[];
    items.forEach((item, i) => {
      animate(item, { opacity: [0, 1], y: [4, 0] }, { duration: 0.18, delay: i * 0.03 });
    });
  });
</script>

<section class="flex min-w-0 flex-col overflow-hidden rounded-lg border border-border bg-card">
  <header class="flex items-center gap-2 border-b border-border px-2.5 py-2">
    <span
      class="grid size-5 shrink-0 place-items-center rounded-[5px] bg-foreground text-[11px] font-bold text-background tabular-nums"
    >
      {n}
    </span>
    <span class="min-w-0 flex-1 truncate text-[13px] font-semibold">
      {$_(roleLabel(role.position)).replace(/^POS\s*\d+\s*—\s*|^ПОЗ\s*\d+\s*—\s*/u, "")}
    </span>
    <span class="shrink-0 font-mono text-[11px] tabular-nums text-muted-foreground">
      {pct(role.role_winrate)}
    </span>
  </header>

  <div bind:this={listEl} class="flex flex-col gap-0.5 p-1.5">
    {#each role.heroes as h, i (h.hero_id)}
      <HeroCard hero={h} rank={i + 1} />
    {/each}
    {#if role.heroes.length === 0}
      <p class="px-2 py-6 text-center text-xs text-muted-foreground">—</p>
    {/if}
  </div>
</section>
