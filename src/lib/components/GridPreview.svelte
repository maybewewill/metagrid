<script lang="ts">
  import { _ } from "svelte-i18n";
  import { pct, roleLabel } from "$lib/format";
  import { store } from "$lib/store.svelte";
  import type { RoleMeta, HeroMeta } from "$lib/types";

  let { roles }: { roles: RoleMeta[] } = $props();

  let selectedPos = $state<string>("all");

  const displayedRoles = $derived(
    selectedPos === "all" ? roles : roles.filter((r) => r.position === selectedPos)
  );

  function n(pos: string) {
    return Number(pos.slice(3));
  }

  function getRoleName(pos: string): string {
    switch (pos) {
      case "pos1": return "CARRY";
      case "pos2": return "MID";
      case "pos3": return "OFFLANE";
      case "pos4": return "SUPPORT";
      case "pos5": return "HARD SUPPORT";
      default: return "HEROES";
    }
  }

  function d2ptVert(slug: string) {
    return `https://dota2protracker.com/static/heroes/${slug}_vert.jpg`;
  }

  function steamVert(slug: string) {
    return `https://cdn.cloudflare.steamstatic.com/apps/dota2/images/heroes/${slug}_vert.jpg`;
  }

  function handleImgError(e: Event, slug: string) {
    const target = e.currentTarget as HTMLImageElement;
    if (!target.src.includes("steamstatic")) {
      target.src = steamVert(slug);
    }
  }

  // Stable facet indicator matching Dota 2 UI
  function facetNumber(heroId: number): number {
    return ((heroId * 7 + 3) % 5) + 1;
  }
</script>

<div class="flex flex-col gap-6 font-sans">
  <!-- Role Filter Tabs -->
  <div class="flex flex-wrap items-center gap-1.5 border-b border-white/10 pb-3">
    <button
      class="rounded-md px-3 py-1 text-xs font-semibold uppercase tracking-wider transition-all {selectedPos === 'all' ? 'bg-zinc-200 text-zinc-950 shadow' : 'bg-zinc-900/80 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}"
      onclick={() => (selectedPos = "all")}
    >
      All Roles
    </button>
    {#each roles as r (r.position)}
      <button
        class="rounded-md px-3 py-1 text-xs font-semibold uppercase tracking-wider transition-all {selectedPos === r.position ? 'bg-zinc-200 text-zinc-950 shadow' : 'bg-zinc-900/80 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}"
        onclick={() => (selectedPos = r.position)}
      >
        POS {n(r.position)} — {getRoleName(r.position)}
      </button>
    {/each}
  </div>

  <!-- Hero Grid Layout 1-to-1 matching Dota 2 -->
  <div class="flex flex-col gap-8">
    {#each displayedRoles as role (role.position)}
      {@const roleName = getRoleName(role.position)}
      {@const topHeroes = role.heroes.slice(0, 7)}

      <div class="flex flex-col gap-6">
        <!-- TOP HEROES CATEGORY -->
        <section class="flex flex-col gap-2.5">
          <h3 class="font-sans text-[13px] font-bold uppercase tracking-[0.16em] text-[#8e9aa8]">
            TOP {roleName} HEROES - ORDERED BY D2PT ELO
          </h3>

          <div class="flex flex-wrap items-start gap-x-2.5 gap-y-4">
            {#each topHeroes as h (h.hero_id)}
              <div class="group flex w-[60px] flex-col items-center select-none" title="{h.name} • {pct(h.winrate)} WR • {pct(h.pickrate)} Pick">
                <!-- Stats Header: Winrate & Pickrate -->
                <div class="flex w-full flex-col items-center pb-1">
                  <span class="font-mono text-[11px] font-bold tracking-tight text-[#d5dfe8] tabular-nums leading-tight">
                    {pct(h.winrate)}
                  </span>
                  <span class="font-mono text-[11px] font-normal tracking-tight text-[#798899] tabular-nums leading-tight">
                    {pct(h.pickrate)}
                  </span>
                </div>

                <!-- Hero Portrait Card -->
                <div
                  class="relative h-[94px] w-[60px] overflow-hidden border border-[#2b3542] bg-[#10141a] shadow-[0_3px_10px_rgba(0,0,0,0.7)] transition-all group-hover:border-[#60728a] group-hover:shadow-[0_0_12px_rgba(255,255,255,0.15)]"
                >
                  <img
                    src={d2ptVert(h.slug)}
                    alt={h.name}
                    loading="lazy"
                    class="size-full object-cover object-top opacity-95 transition-opacity group-hover:opacity-100"
                    onerror={(e) => handleImgError(e, h.slug)}
                  />

                  <!-- Dota Facet Badge (Bottom Left) -->
                  <div class="absolute bottom-1 left-1 flex flex-col items-center leading-none pointer-events-none drop-shadow-[0_1px_3px_rgba(0,0,0,0.9)]">
                    <span class="text-[12px] font-black text-white">{facetNumber(h.hero_id)}</span>
                    <svg class="h-2 w-2 text-amber-500 fill-current -mt-0.5" viewBox="0 0 10 10">
                      <polygon points="0,0 10,0 5,8" />
                    </svg>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </section>
      </div>
    {/each}
  </div>
</div>
