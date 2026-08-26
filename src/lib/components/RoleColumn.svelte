<script lang="ts">
  import { _ } from "svelte-i18n";
  import HeroCard from "$lib/components/HeroCard.svelte";
  import RoleIcon from "$lib/components/RoleIcon.svelte";
  import { pct, roleLabel } from "$lib/format";
  import { prefersReducedMotion } from "$lib/motion";
  import { animate } from "motion";
  import { store } from "$lib/store.svelte";
  import type { RoleMeta } from "$lib/types";

  let { role }: { role: RoleMeta } = $props();

  const n = $derived(Number(role.position.slice(3)));

  const title = $derived.by(() => {
    if (store.settings?.role_labels === "pos") {
      return `${$_("pos_prefix")} ${n}`;
    }
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
  class="flex h-full min-w-0 flex-col overflow-hidden rounded-sm border border-zinc-800/80 bg-zinc-950/80 shadow-md"
>
  <header class="flex items-center gap-2 border-b border-zinc-800 bg-zinc-900/60 px-3 py-2.5">
    <RoleIcon position={role.position} class="size-4 text-zinc-300 shrink-0" />
    <span class="min-w-0 flex-1 truncate text-[13px] font-bold tracking-wider uppercase text-zinc-200">{title}</span>
    <span class="shrink-0 font-mono text-[11.5px] font-semibold tabular-nums text-zinc-400">
      {pct(role.role_winrate)}
    </span>
  </header>

  <div bind:this={listEl} class="scroll-thin flex min-h-0 flex-1 flex-col justify-between gap-1 overflow-auto p-1.5 bg-zinc-950/40">
    {#each role.heroes.slice(0, 7) as h (h.hero_id)}
      <HeroCard hero={h} />
    {/each}
    {#if role.heroes.length === 0}
      <p class="px-2 py-6 text-center text-xs text-muted-foreground">—</p>
    {/if}
  </div>
</section>
