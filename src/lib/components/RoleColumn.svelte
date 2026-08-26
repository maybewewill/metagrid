<script lang="ts">
  import { _ } from "svelte-i18n";
  import { Card, CardHeader, CardContent } from "$lib/components/ui/card";
  import HeroCard from "$lib/components/HeroCard.svelte";
  import { pct, roleLabel } from "$lib/format";
  import { hoverLift, prefersReducedMotion } from "$lib/motion";
  import { animate } from "motion";
  import type { RoleMeta } from "$lib/types";

  let { role }: { role: RoleMeta } = $props();

  const n = $derived(Number(role.position.slice(3)));

  let listEl = $state<HTMLDivElement | null>(null);

  // Light mount stagger for the hero rows — skipped entirely under
  // reduced-motion.
  $effect(() => {
    if (!listEl || prefersReducedMotion()) return;
    const items = Array.from(listEl.children) as HTMLElement[];
    items.forEach((item, i) => {
      animate(item, { opacity: [0, 1], y: [6, 0] }, { duration: 0.2, delay: i * 0.04 });
    });
  });
</script>

<div use:hoverLift>
  <Card class="gap-0 overflow-hidden py-0" data-slot="role-column">
    <div class="h-1" style={`background: var(--pos${n})`}></div>
    <CardHeader class="flex-row items-baseline justify-between border-b border-border py-2">
      <span class="text-sm font-semibold">{$_(roleLabel(role.position))}</span>
      <span class="text-xs text-muted-foreground">{pct(role.role_winrate)}</span>
    </CardHeader>
    <CardContent bind:ref={listEl} class="flex flex-col gap-1.5 py-2">
      {#each role.heroes as h, i (h.hero_id)}
        <HeroCard hero={h} rank={i + 1} />
      {/each}
    </CardContent>
  </Card>
</div>
