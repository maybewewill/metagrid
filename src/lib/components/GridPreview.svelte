<script lang="ts">
  import { _ } from "svelte-i18n";
  import type { MetaSnapshot, GridConfig, Position, MetaMode } from "$lib/types";
  import { getHeroById } from "$lib/heroes";
  import { getHeroPortraitUrl } from "$lib/utils";
  import { store } from "$lib/store.svelte";
  import RoleIcon from "$lib/components/RoleIcon.svelte";
  import { ChevronDown, Database, Check, Layers } from "@lucide/svelte";

  type TabKey = Position | "all_roles";

  let { snapshot }: { snapshot: MetaSnapshot } = $props();

  const positions: { key: Position; posNum: number; roleName: string; searchTerms: string[] }[] = [
    { key: "Pos1", posNum: 1, roleName: "Carry", searchTerms: ["carry", "pos 1", "pos1"] },
    { key: "Pos2", posNum: 2, roleName: "Mid", searchTerms: ["mid", "pos 2", "pos2"] },
    { key: "Pos3", posNum: 3, roleName: "Offlane", searchTerms: ["offlane", "pos 3", "pos3"] },
    { key: "Pos4", posNum: 4, roleName: "Support", searchTerms: ["support", "pos 4", "pos4"] },
    { key: "Pos5", posNum: 5, roleName: "Hard Support", searchTerms: ["hard support", "pos 5", "pos5"] },
  ];

  const metaModes: { id: MetaMode }[] = [
    { id: "matches" },
    { id: "matches_wr" },
    { id: "d2ptrating" },
  ];

  let selectedTab = $state<TabKey>("Pos1");
  let sourceMenuOpen = $state(false);

  const currentMetaMode = $derived<MetaMode>(
    store.settings?.meta_mode || "matches"
  );

  async function selectMetaMode(mode: MetaMode) {
    sourceMenuOpen = false;
    if (store.settings?.meta_mode === mode) return;
    await store.saveSettings({ meta_mode: mode });
    await store.fetchOnly();
  }

  const currentPosInfo = $derived(
    positions.find((p) => p.key === selectedTab) || positions[0]
  );

  const activeRoleConfig = $derived.by<GridConfig | null>(() => {
    if (selectedTab === "all_roles") return null;
    if (!snapshot.configs || snapshot.configs.length === 0) return null;
    const info = currentPosInfo;
    if (info.key === "Pos5") {
      const hs = snapshot.configs.find((c) => {
        const name = c.config_name.toLowerCase();
        return name.includes("hard support") || name.includes("pos 5") || name.includes("pos5");
      });
      if (hs) return hs;
    }
    if (info.key === "Pos4") {
      const s = snapshot.configs.find((c) => {
        const name = c.config_name.toLowerCase();
        return !name.includes("hard support") && (name.includes("support") || name.includes("pos 4") || name.includes("pos4"));
      });
      if (s) return s;
    }
    return (
      snapshot.configs.find((c) => {
        const name = c.config_name.toLowerCase();
        return !name.includes("all roles") && info.searchTerms.some((t) => name.includes(t));
      }) || null
    );
  });

  const structuredRows = $derived.by(() => {
    if (selectedTab === "all_roles" || !activeRoleConfig) return null;
    const topCat = activeRoleConfig.categories.find(
      (c) =>
        c.category_name.toLowerCase().startsWith("top heroes") ||
        c.category_name.toLowerCase().startsWith("top")
    );
    if (!topCat || topCat.hero_ids.length === 0) return null;

    const matchupCats = activeRoleConfig.categories.filter((c) => c !== topCat);
    return topCat.hero_ids.map((heroId, index) => {
      const rowCats = matchupCats.slice(index * 4, (index + 1) * 4);
      return {
        heroId,
        rank: index + 1,
        bestWith: rowCats[0]?.hero_ids || [],
        worstWith: rowCats[1]?.hero_ids || [],
        bestAgainst: rowCats[2]?.hero_ids || [],
        worstAgainst: rowCats[3]?.hero_ids || [],
      };
    });
  });

  const allRolesData = $derived.by(() => {
    const rolesList: { posKey: Position; title: string; heroIds: number[] }[] = [];
    const allRolesConfig = snapshot.configs?.find((c) => c.config_name.toLowerCase().includes("all roles"));

    for (const p of positions) {
      let heroIds: number[] = [];
      if (allRolesConfig) {
        const cat = allRolesConfig.categories.find((c) => {
          const cn = c.category_name.toLowerCase();
          return (
            cn === p.roleName.toLowerCase() ||
            cn === p.key.toLowerCase() ||
            cn.includes(`pos ${p.posNum}`) ||
            cn.includes(`pos${p.posNum}`)
          );
        });
        if (cat) {
          heroIds = cat.hero_ids.slice(0, 7);
        }
      }
      if (heroIds.length === 0 && snapshot.configs) {
        const roleCfg = snapshot.configs.find((c) => {
          const cn = c.config_name.toLowerCase();
          return !cn.includes("all roles") && p.searchTerms.some((t) => cn.includes(t));
        });
        const topCat = roleCfg?.categories.find((c) => c.category_name.toLowerCase().startsWith("top"));
        if (topCat) {
          heroIds = topCat.hero_ids.slice(0, 7);
        }
      }
      if (heroIds.length === 0) {
        const rMeta = snapshot.roles.find((r) => r.position === p.key);
        if (rMeta) {
          heroIds = rMeta.heroes.slice(0, 7).map((h) => h.hero_id);
        }
      }
      rolesList.push({
        posKey: p.key,
        title: `META ${$_(`role_upper.${p.key.toLowerCase()}`)}`,
        heroIds,
      });
    }
    return rolesList;
  });

  const activeRoleMeta = $derived(
    snapshot.roles.find((r) => r.position === selectedTab) || null
  );

  function handleImgError(e: Event) {
    const target = e.currentTarget as HTMLImageElement;
    target.style.visibility = "hidden";
  }
</script>

<div class="flex flex-col gap-3 font-sans">
  <div class="flex flex-wrap items-center justify-between gap-2 border-b border-zinc-800/80 pb-2">
    <div class="flex flex-wrap items-center gap-1.5">
      {#each positions as p (p.key)}
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-sm px-3.5 py-1.5 text-xs font-bold uppercase tracking-wider transition-all {selectedTab === p.key ? 'bg-zinc-100 text-zinc-950 shadow-xs' : 'bg-zinc-900/90 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}"
          onclick={() => (selectedTab = p.key)}
        >
          <RoleIcon position={p.key} class="size-3.5" />
          <span>{$_(`role_upper.${p.key.toLowerCase()}`)}</span>
        </button>
      {/each}

      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-sm px-3.5 py-1.5 text-xs font-bold uppercase tracking-wider transition-all {selectedTab === 'all_roles' ? 'bg-zinc-100 text-zinc-950 shadow-xs' : 'bg-zinc-900/90 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200'}"
        onclick={() => (selectedTab = 'all_roles')}
      >
        <Layers class="size-3.5" />
        <span>{$_("role_upper.all_roles")}</span>
      </button>
    </div>

    <div class="relative shrink-0">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-sm border border-zinc-800/60 bg-zinc-950 px-3 py-1.5 text-xs font-bold uppercase tracking-wider text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200 hover:border-zinc-700/70 transition-all shadow-xs"
        onclick={() => (sourceMenuOpen = !sourceMenuOpen)}
        disabled={store.loading}
      >
        <Database size={13} class="text-zinc-500" />
        <span>{$_(`settings.meta_mode_${currentMetaMode}`)}</span>
        <ChevronDown size={13} class="text-zinc-500 transition-transform duration-200 {sourceMenuOpen ? 'rotate-180' : ''}" />
      </button>

      {#if sourceMenuOpen}
        <div
          class="fixed inset-0 z-40"
          role="button"
          tabindex="0"
          aria-label="close menu"
          onclick={() => (sourceMenuOpen = false)}
          onkeydown={(e) => e.key === 'Escape' && (sourceMenuOpen = false)}
        ></div>

        <div class="absolute right-0 top-full mt-1 z-50 min-w-[200px] rounded-sm border border-zinc-800 bg-zinc-950 p-1 shadow-xl">
          <div class="px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-zinc-500 border-b border-zinc-800/80 mb-1">
            {$_("settings.meta_mode")}
          </div>
          {#each metaModes as mode (mode.id)}
            <button
              type="button"
              class="flex w-full items-center justify-between gap-2 rounded-xs px-2 py-1.5 text-left text-xs transition-colors {currentMetaMode === mode.id ? 'bg-zinc-900 text-zinc-100 font-semibold' : 'text-zinc-400 hover:bg-zinc-900/60 hover:text-zinc-200'}"
              onclick={() => selectMetaMode(mode.id)}
            >
              <span>{$_(`settings.meta_mode_${mode.id}`)}</span>
              {#if currentMetaMode === mode.id}
                <Check size={13} class="text-zinc-200" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  {#if selectedTab === "all_roles"}
    <div class="flex flex-col gap-1.5">
      {#each allRolesData as roleSec (roleSec.posKey)}
        <div class="flex items-center gap-3.5 rounded-sm border border-zinc-800/60 bg-zinc-950/70 px-3.5 py-1.5 shadow-xs transition-colors hover:border-zinc-700/60">
          <div class="flex items-center gap-2 w-[140px] shrink-0 border-r border-zinc-800/60 pr-2.5">
            <RoleIcon position={roleSec.posKey} class="size-4 shrink-0" />
            <span class="font-sans text-xs font-bold uppercase tracking-wider text-zinc-300">
              {$_(`role_upper.${roleSec.posKey.toLowerCase()}`)}
            </span>
          </div>

          <div class="flex items-center gap-1.5 flex-1 min-w-0">
            {#each roleSec.heroIds as heroId, index (heroId)}
              {@const hero = getHeroById(heroId)}
              <div
                class="group relative size-auto shrink-0 overflow-hidden rounded-xs border border-zinc-700/70 bg-zinc-900 shadow-sm transition-all hover:scale-105 hover:border-zinc-400 hover:shadow-[0_0_10px_rgba(255,255,255,0.15)] hover:z-10 select-none"
                title="#{index + 1} {hero.name}"
              >
                {#if hero.slug}
                  <img
                    src={getHeroPortraitUrl(hero.slug)}
                    alt={hero.name}
                    loading="lazy"
                    class="h-[78px] w-[54px] object-cover object-top opacity-95 transition-opacity group-hover:opacity-100"
                    onerror={handleImgError}
                  />
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {:else if structuredRows && structuredRows.length > 0}
    <div class="flex flex-col gap-3">
      {#each structuredRows as row (row.heroId)}
        {@const hero = getHeroById(row.heroId)}
        <div class="flex flex-col xl:flex-row items-stretch gap-3.5 rounded-sm border border-zinc-800/70 bg-zinc-950/70 p-3 shadow-xs transition-colors hover:border-zinc-700/80">
          <div class="flex flex-col items-center justify-start gap-1.5 shrink-0 w-full xl:w-[110px] border-b xl:border-b-0 xl:border-r border-zinc-800/60 pb-2 xl:pb-0 xl:pr-3.5">
            <div class="relative h-[106px] w-[74px] overflow-hidden rounded-sm border border-zinc-700/80 bg-zinc-900 shadow-md transition-transform hover:scale-105">
              {#if hero.slug}
                <img
                  src={getHeroPortraitUrl(hero.slug)}
                  alt={hero.name}
                  loading="lazy"
                  class="size-full object-cover object-top opacity-95 transition-opacity hover:opacity-100"
                  onerror={handleImgError}
                />
              {/if}
            </div>
            <div class="flex flex-col items-center justify-center text-center w-full px-0.5">
              <span class="font-mono text-[11px] font-bold text-zinc-400 leading-none">
                #{row.rank}
              </span>
              <span class="text-xs font-semibold text-zinc-200 leading-snug line-clamp-2 break-words mt-0.5" title={hero.name}>
                {hero.name}
              </span>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-2.5 flex-1 min-w-0">
            <div class="flex flex-col gap-1.5 rounded-sm border border-emerald-900/30 bg-emerald-950/20 p-2">
              <div class="flex items-center justify-between">
                <span class="text-[11px] font-bold uppercase tracking-wider text-emerald-400">
                  Best with
                </span>
                <span class="font-mono text-[10px] text-emerald-500/80">{row.bestWith.length}</span>
              </div>
              <div class="flex flex-wrap items-center gap-1.5 pt-0.5">
                {#each row.bestWith as hId (hId)}
                  {@const h = getHeroById(hId)}
                  <div
                    class="group relative size-auto shrink-0 overflow-hidden rounded-xs border border-emerald-800/40 bg-zinc-900 transition-all hover:scale-110 hover:border-emerald-400 hover:shadow-[0_0_8px_rgba(52,211,153,0.4)] hover:z-10"
                    title={h.name}
                  >
                    {#if h.slug}
                      <img
                        src={getHeroPortraitUrl(h.slug)}
                        alt={h.name}
                        loading="lazy"
                        class="h-[60px] w-[42px] object-cover object-top"
                        onerror={handleImgError}
                      />
                    {/if}
                  </div>
                {/each}
              </div>
            </div>

            <div class="flex flex-col gap-1.5 rounded-sm border border-rose-900/30 bg-rose-950/20 p-2">
              <div class="flex items-center justify-between">
                <span class="text-[11px] font-bold uppercase tracking-wider text-rose-400">
                  Worst with
                </span>
                <span class="font-mono text-[10px] text-rose-500/80">{row.worstWith.length}</span>
              </div>
              <div class="flex flex-wrap items-center gap-1.5 pt-0.5">
                {#each row.worstWith as hId (hId)}
                  {@const h = getHeroById(hId)}
                  <div
                    class="group relative size-auto shrink-0 overflow-hidden rounded-xs border border-rose-800/40 bg-zinc-900 transition-all hover:scale-110 hover:border-rose-400 hover:shadow-[0_0_8px_rgba(251,113,133,0.4)] hover:z-10"
                    title={h.name}
                  >
                    {#if h.slug}
                      <img
                        src={getHeroPortraitUrl(h.slug)}
                        alt={h.name}
                        loading="lazy"
                        class="h-[60px] w-[42px] object-cover object-top"
                        onerror={handleImgError}
                      />
                    {/if}
                  </div>
                {/each}
              </div>
            </div>

            <div class="flex flex-col gap-1.5 rounded-sm border border-emerald-900/30 bg-emerald-950/20 p-2">
              <div class="flex items-center justify-between">
                <span class="text-[11px] font-bold uppercase tracking-wider text-emerald-400">
                  Best against
                </span>
                <span class="font-mono text-[10px] text-emerald-500/80">{row.bestAgainst.length}</span>
              </div>
              <div class="flex flex-wrap items-center gap-1.5 pt-0.5">
                {#each row.bestAgainst as hId (hId)}
                  {@const h = getHeroById(hId)}
                  <div
                    class="group relative size-auto shrink-0 overflow-hidden rounded-xs border border-emerald-800/40 bg-zinc-900 transition-all hover:scale-110 hover:border-emerald-400 hover:shadow-[0_0_8px_rgba(52,211,153,0.4)] hover:z-10"
                    title={h.name}
                  >
                    {#if h.slug}
                      <img
                        src={getHeroPortraitUrl(h.slug)}
                        alt={h.name}
                        loading="lazy"
                        class="h-[60px] w-[42px] object-cover object-top"
                        onerror={handleImgError}
                      />
                    {/if}
                  </div>
                {/each}
              </div>
            </div>

            <div class="flex flex-col gap-1.5 rounded-sm border border-rose-900/30 bg-rose-950/20 p-2">
              <div class="flex items-center justify-between">
                <span class="text-[11px] font-bold uppercase tracking-wider text-rose-400">
                  Worst against
                </span>
                <span class="font-mono text-[10px] text-rose-500/80">{row.worstAgainst.length}</span>
              </div>
              <div class="flex flex-wrap items-center gap-1.5 pt-0.5">
                {#each row.worstAgainst as hId (hId)}
                  {@const h = getHeroById(hId)}
                  <div
                    class="group relative size-auto shrink-0 overflow-hidden rounded-xs border border-rose-800/40 bg-zinc-900 transition-all hover:scale-110 hover:border-rose-400 hover:shadow-[0_0_8px_rgba(251,113,133,0.4)] hover:z-10"
                    title={h.name}
                  >
                    {#if h.slug}
                      <img
                        src={getHeroPortraitUrl(h.slug)}
                        alt={h.name}
                        loading="lazy"
                        class="h-[60px] w-[42px] object-cover object-top"
                        onerror={handleImgError}
                      />
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else if activeRoleConfig}
    <div class="flex flex-col gap-4">
      {#each activeRoleConfig.categories as cat (cat.category_name)}
        <section class="flex flex-col gap-2 rounded-sm border border-zinc-800/60 bg-zinc-950/60 p-3 shadow-xs">
          <div class="flex items-center justify-between border-b border-zinc-800/40 pb-2">
            <h3 class="font-sans text-xs font-bold uppercase tracking-widest text-zinc-300">
              {cat.category_name}
            </h3>
            <span class="font-mono text-[11px] font-semibold text-zinc-500">{cat.hero_ids.length}</span>
          </div>
          <div class="flex flex-wrap items-start gap-2 pt-1">
            {#each cat.hero_ids as heroId (heroId)}
              {@const hero = getHeroById(heroId)}
              <div class="group relative flex flex-col items-center select-none" title={hero.name}>
                <div class="relative h-[78px] w-[54px] overflow-hidden rounded-sm border border-zinc-700/60 bg-zinc-900 shadow-md transition-all group-hover:border-blue-400 group-hover:shadow-[0_0_12px_rgba(96,165,250,0.35)] group-hover:scale-105">
                  {#if hero.slug}
                    <img
                      src={getHeroPortraitUrl(hero.slug)}
                      alt={hero.name}
                      loading="lazy"
                      class="size-full object-cover object-top opacity-95 transition-opacity group-hover:opacity-100"
                      onerror={handleImgError}
                    />
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  {:else if activeRoleMeta}
    <div class="flex flex-wrap items-start gap-2 rounded-sm border border-zinc-800/60 bg-zinc-950/60 p-3">
      {#each activeRoleMeta.heroes as hero (hero.hero_id)}
        <div class="group relative flex flex-col items-center select-none" title="{hero.name}">
          <div class="relative h-[78px] w-[54px] overflow-hidden rounded-sm border border-zinc-700/60 bg-zinc-900 shadow-md transition-all group-hover:border-blue-400 group-hover:scale-105">
            {#if hero.slug}
              <img
                src={getHeroPortraitUrl(hero.slug)}
                alt={hero.name}
                loading="lazy"
                class="size-full object-cover object-top"
                onerror={handleImgError}
              />
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
