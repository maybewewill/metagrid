<script lang="ts">
  import { _ } from "svelte-i18n";
  import { pct2 } from "$lib/format";
  import { store } from "$lib/store.svelte";
  import RoleIcon from "$lib/components/RoleIcon.svelte";
  import type { RoleMeta } from "$lib/types";
  import { getHeroPortraitUrl } from "$lib/utils";

  let { roles }: { roles: RoleMeta[] } = $props();

  let selectedPos = $state<string>("pos1");

  const currentRole = $derived(
    roles.find((r) => r.position === selectedPos) || roles[0]
  );

  function n(pos: string) {
    return Number(pos.slice(3));
  }

  function tabLabel(r: RoleMeta) {
    if (store.settings?.role_labels === "pos") {
      return `${$_("pos_prefix")} ${n(r.position)}`;
    }
    return $_(`role_upper.${r.position.toLowerCase()}`);
  }

  function sectionRole(r: RoleMeta) {
    if (store.settings?.role_labels === "pos") {
      return `${$_("pos_prefix")} ${n(r.position)}`;
    }
    return $_(`role_upper.${r.position.toLowerCase()}`);
  }

  function handleImgError(e: Event) {
    const target = e.currentTarget as HTMLImageElement;
    target.style.visibility = "hidden";
  }
</script>

<div class="flex flex-col gap-5 font-sans">
  <div class="flex flex-wrap items-center gap-2 border-b border-white/10 pb-3">
    {#each roles as r (r.position)}
      <button
        class="inline-flex items-center gap-2 rounded-sm px-3.5 py-2 text-xs font-bold uppercase tracking-wider transition-all {selectedPos === r.position ? 'bg-zinc-200 text-zinc-950 shadow-sm' : 'bg-zinc-900/80 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}"
        onclick={() => (selectedPos = r.position)}
      >
        <RoleIcon position={r.position} class="size-3.5" />
        <span>{tabLabel(r)}</span>
      </button>
    {/each}
  </div>

  {#if currentRole}
    {@const roleUpper = sectionRole(currentRole)}
    {@const topHeroes = currentRole.heroes.filter((h) => h.is_top).length > 0 ? currentRole.heroes.filter((h) => h.is_top) : currentRole.heroes.slice(0, 7)}
    {@const topHeroIds = new Set(topHeroes.map((h) => h.hero_id))}
    {@const otherHeroes = currentRole.heroes.filter((h) => !topHeroIds.has(h.hero_id))}

    <div class="flex flex-col gap-6">
      <!-- TOP HEROES CATEGORY -->
      <section class="flex flex-col gap-3">
        <h3 class="font-sans text-[13px] font-bold uppercase tracking-[0.14em] text-[#8e9aa8]">
          {$_("grid.top", { values: { role: roleUpper } })}
        </h3>

        <div class="flex flex-wrap items-start gap-x-3 gap-y-4">
          {#each topHeroes as h (h.hero_id)}
            <div class="group flex w-[68px] flex-col items-center select-none" title="{h.name} • {pct2(h.winrate)} WR • {pct2(h.pickrate)} Pick">
              <!-- Stats Header: Winrate & Pickrate (Exact 2 decimals) -->
              <div class="flex w-full flex-col items-center pb-1">
                <span class="font-sans text-[13px] font-bold tracking-tight text-[#e2e8f0] tabular-nums leading-tight">
                  {pct2(h.winrate)}
                </span>
                <span class="font-sans text-[11.5px] font-medium tracking-tight text-[#8e9aa8] tabular-nums leading-tight">
                  {pct2(h.pickrate)}
                </span>
              </div>

              <!-- Hero Portrait Card (No Level / Facet Badge) -->
              <div
                class="relative h-[104px] w-[68px] overflow-hidden rounded-sm border border-[#2b3542] bg-[#10141a] shadow-[0_3px_10px_rgba(0,0,0,0.8)] transition-all group-hover:border-[#60a5fa] group-hover:shadow-[0_0_10px_rgba(96,165,250,0.3)]"
              >
                <img
                  src={getHeroPortraitUrl(h.slug)}
                  alt={h.name}
                  loading="lazy"
                  class="size-full object-cover object-top opacity-95 transition-opacity group-hover:opacity-100"
                  onerror={handleImgError}
                />
              </div>
            </div>
          {/each}
        </div>
      </section>

      <!-- OTHER HEROES CATEGORY -->
      {#if otherHeroes.length > 0}
        <section class="flex flex-col gap-3">
          <h3 class="font-sans text-[13px] font-bold uppercase tracking-[0.14em] text-[#8e9aa8]">
            {$_("grid.other", { values: { role: roleUpper } })}
          </h3>

          <div class="flex flex-wrap items-start gap-x-3 gap-y-4">
            {#each otherHeroes as h (h.hero_id)}
              <div class="group flex w-[68px] flex-col items-center select-none" title="{h.name} • {pct2(h.winrate)} WR • {pct2(h.pickrate)} Pick">
                <!-- Stats Header: Winrate & Pickrate (Exact 2 decimals) -->
                <div class="flex w-full flex-col items-center pb-1">
                  <span class="font-sans text-[13px] font-bold tracking-tight text-[#e2e8f0] tabular-nums leading-tight">
                    {pct2(h.winrate)}
                  </span>
                  <span class="font-sans text-[11.5px] font-medium tracking-tight text-[#8e9aa8] tabular-nums leading-tight">
                    {pct2(h.pickrate)}
                  </span>
                </div>

                <!-- Hero Portrait Card (No Level / Facet Badge) -->
                <div
                  class="relative h-[104px] w-[68px] overflow-hidden rounded-sm border border-[#2b3542] bg-[#10141a] shadow-[0_3px_10px_rgba(0,0,0,0.8)] transition-all group-hover:border-[#60a5fa] group-hover:shadow-[0_0_10px_rgba(96,165,250,0.3)]"
                >
                  <img
                    src={getHeroPortraitUrl(h.slug)}
                    alt={h.name}
                    loading="lazy"
                    class="size-full object-cover object-top opacity-95 transition-opacity group-hover:opacity-100"
                    onerror={handleImgError}
                  />
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/if}
    </div>
  {/if}
</div>
