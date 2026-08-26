<script lang="ts">
  import { _ } from "svelte-i18n";
  import { Card, CardHeader, CardContent } from "$lib/components/ui/card";
  import HeroCard from "$lib/components/HeroCard.svelte";
  import { pct, roleLabel } from "$lib/format";
  import type { RoleMeta } from "$lib/types";

  let { role }: { role: RoleMeta } = $props();

  const n = $derived(Number(role.position.slice(3)));
</script>

<Card class="gap-0 overflow-hidden py-0" data-slot="role-column">
  <div class="h-1" style={`background: var(--pos${n})`}></div>
  <CardHeader class="flex-row items-baseline justify-between border-b border-border py-2">
    <span class="text-sm font-semibold">{$_(roleLabel(role.position))}</span>
    <span class="text-xs text-muted-foreground">{pct(role.role_winrate)}</span>
  </CardHeader>
  <CardContent class="flex flex-col gap-1.5 py-2">
    {#each role.heroes as h, i (h.hero_id)}
      <HeroCard hero={h} rank={i + 1} />
    {/each}
  </CardContent>
</Card>
