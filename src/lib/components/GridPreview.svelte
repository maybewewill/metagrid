<script lang="ts">
  import { _ } from "svelte-i18n";
  import { pct, roleLabel } from "$lib/format";
  import { store } from "$lib/store.svelte";
  import type { RoleMeta, HeroMeta } from "$lib/types";

  let { roles }: { roles: RoleMeta[] } = $props();

  function n(pos: string) {
    return Number(pos.slice(3));
  }
  function title(role: RoleMeta) {
    if (store.settings?.role_labels === "pos") return `POS ${n(role.position)}`;
    return $_(roleLabel(role.position)).replace(/^POS\s*\d+\s*—\s*|^ПОЗ\s*\d+\s*—\s*/u, "");
  }
  function vert(h: HeroMeta) {
    return `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/heroes/${h.slug}_vert.jpg`;
  }
  function wrColor(w: number) {
    const t = Math.max(0, Math.min(1, (w - 0.48) / 0.08));
    return `oklch(0.82 0.15 ${20 + t * 130})`;
  }
</script>

<!-- Approximates Dota's in-client hero grid: each role a titled category band
     holding vertical hero portraits. -->
<div class="flex flex-col gap-3">
  {#each roles as role (role.position)}
    <section class="mg-rise rounded-xl border border-border bg-card p-3">
      <header class="mb-2.5 flex items-center gap-2">
        <span
          class="grid size-5 place-items-center rounded-[5px] bg-primary text-[11px] font-bold text-primary-foreground tabular-nums"
        >
          {n(role.position)}
        </span>
        <span class="text-sm font-semibold">{title(role)}</span>
        <span class="ml-auto font-mono text-[11px] tabular-nums text-muted-foreground">
          {pct(role.role_winrate)} WR
        </span>
      </header>

      <div class="flex flex-wrap gap-2">
        {#each role.heroes as h (h.hero_id)}
          <div class="group flex w-[70px] flex-col gap-1" title={h.name}>
            <div
              class="relative aspect-[3/4] w-full overflow-hidden rounded-md bg-muted ring-1 ring-border transition-transform group-hover:scale-[1.04]"
            >
              <img
                src={vert(h)}
                alt={h.name}
                loading="lazy"
                class="size-full object-cover object-top"
              />
              <span
                class="absolute right-0 bottom-0 rounded-tl-md bg-black/75 px-1 py-0.5 text-[10px] font-semibold leading-none tabular-nums"
                style:color={wrColor(h.winrate)}
              >
                {pct(h.winrate)}
              </span>
            </div>
            <span class="truncate text-center text-[10px] text-muted-foreground">{h.name}</span>
          </div>
        {/each}
      </div>
    </section>
  {/each}
</div>
