<script lang="ts">
  import { _ } from "svelte-i18n";
  import HeroCard from "$lib/components/HeroCard.svelte";
  import { pct, roleLabel } from "$lib/format";
  import { prefersReducedMotion } from "$lib/motion";
  import { animate } from "motion";
  import { store } from "$lib/store.svelte";
  import type { RoleMeta } from "$lib/types";

  let { role }: { role: RoleMeta } = $props();

  const n = $derived(Number(role.position.slice(3)));

  const title = $derived.by(() => {
    if (store.settings?.role_labels === "pos") return `POS ${n}`;
    // named: strip the "POS 1 — " / "ПОЗ 1 — " prefix, keep just the role name.
    return $_(roleLabel(role.position)).replace(/^POS\s*\d+\s*—\s*|^ПОЗ\s*\d+\s*—\s*/u, "");
  });

  let listEl = $state<HTMLDivElement | null>(null);
  $effect(() => {
    if (!listEl || prefersReducedMotion()) return;
    const items = Array.from(listEl.children) as HTMLElement[];
    items.forEach((item, i) => {
      animate(item, { opacity: [0, 1], y: [4, 0] }, { duration: 0.18, delay: i * 0.025 });
    });
  });
</script>

<section
  class="flex min-w-0 flex-col overflow-hidden rounded-xl border border-border bg-card shadow-sm"
>
  <header class="flex items-center gap-2 border-b border-border px-2.5 py-2">
    <span
      class="grid size-5 shrink-0 place-items-center rounded-[5px] bg-primary text-[11px] font-bold tabular-nums text-primary-foreground"
    >
      {n}
    </span>
    <span class="min-w-0 flex-1 truncate text-[13px] font-semibold">{title}</span>
    <span class="shrink-0 font-mono text-[11px] tabular-nums text-muted-foreground">
      {pct(role.role_winrate)}
    </span>
  </header>

  <div bind:this={listEl} class="scroll-thin flex min-h-0 flex-1 flex-col gap-0.5 overflow-auto p-1.5">
    {#each role.heroes.slice(0, 7) as h, i (h.hero_id)}
      <HeroCard hero={h} rank={i + 1} />
    {/each}
    {#if role.heroes.length === 0}
      <p class="px-2 py-6 text-center text-xs text-muted-foreground">—</p>
    {/if}
  </div>
</section>
