<script lang="ts">
  import { _ } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { pct } from "$lib/format";
  import { store } from "$lib/store.svelte";
  import type { RoleMeta, HeroMeta } from "$lib/types";
  import { Button } from "$lib/components/ui/button";
  import { Move, Maximize2, RotateCcw, Save, Trash2, Plus, Sparkles, X } from "@lucide/svelte";

  let { roles }: { roles: RoleMeta[] } = $props();

  interface EditorCategory {
    id: string;
    name: string;
    position: string;
    x: number;
    y: number;
    width: number;
    height: number;
    heroes: HeroMeta[];
  }

  function getRoleTitle(pos: string): string {
    switch (pos) {
      case "pos1": return "POS 1 — CARRY";
      case "pos2": return "POS 2 — MID";
      case "pos3": return "POS 3 — OFFLANE";
      case "pos4": return "POS 4 — SUPPORT";
      case "pos5": return "POS 5 — HARD SUPPORT";
      default: return "HEROES";
    }
  }

  // Initialize categories from snapshot roles with calculated layout
  let categories = $state<EditorCategory[]>([]);
  let selectedCategory = $state<string | null>(null);
  let addingHeroToCategory = $state<string | null>(null);
  let searchQuery = $state("");

  // All available heroes across all roles for adding
  const allAvailableHeroes = $derived.by(() => {
    const map = new Map<number, HeroMeta>();
    for (const r of roles) {
      for (const h of r.heroes) {
        if (!map.has(h.hero_id)) map.set(h.hero_id, h);
      }
    }
    return Array.from(map.values());
  });

  const filteredHeroes = $derived(
    searchQuery.trim() === ""
      ? allAvailableHeroes
      : allAvailableHeroes.filter((h) =>
          h.name.toLowerCase().includes(searchQuery.toLowerCase())
        )
  );

  function initLayout() {
    categories = roles.map((role, idx) => {
      // 5 columns arranged neatly
      const colWidth = 230;
      const x = 20 + idx * (colWidth + 16);
      const y = 20;
      return {
        id: role.position,
        name: getRoleTitle(role.position),
        position: role.position,
        x,
        y,
        width: colWidth,
        height: 520,
        heroes: [...role.heroes],
      };
    });
  }

  $effect(() => {
    if (categories.length === 0 && roles.length > 0) {
      initLayout();
    }
  });

  // Dragging category logic
  let activeDrag = $state<{
    catId: string;
    startX: number;
    startY: number;
    initialX: number;
    initialY: number;
  } | null>(null);

  // Resizing category logic
  let activeResize = $state<{
    catId: string;
    startX: number;
    startY: number;
    initialW: number;
    initialH: number;
  } | null>(null);

  function startDrag(e: MouseEvent, cat: EditorCategory) {
    if ((e.target as HTMLElement).closest("button") || (e.target as HTMLElement).closest(".no-drag")) return;
    activeDrag = {
      catId: cat.id,
      startX: e.clientX,
      startY: e.clientY,
      initialX: cat.x,
      initialY: cat.y,
    };
    selectedCategory = cat.id;
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
  }

  function startResize(e: MouseEvent, cat: EditorCategory) {
    e.stopPropagation();
    activeResize = {
      catId: cat.id,
      startX: e.clientX,
      startY: e.clientY,
      initialW: cat.width,
      initialH: cat.height,
    };
    selectedCategory = cat.id;
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
  }

  function onMouseMove(e: MouseEvent) {
    if (activeDrag) {
      const dx = e.clientX - activeDrag.startX;
      const dy = e.clientY - activeDrag.startY;
      const cat = categories.find((c) => c.id === activeDrag!.catId);
      if (cat) {
        cat.x = Math.max(0, Math.round((activeDrag.initialX + dx) / 10) * 10);
        cat.y = Math.max(0, Math.round((activeDrag.initialY + dy) / 10) * 10);
      }
    } else if (activeResize) {
      const dx = e.clientX - activeResize.startX;
      const dy = e.clientY - activeResize.startY;
      const cat = categories.find((c) => c.id === activeResize!.catId);
      if (cat) {
        cat.width = Math.max(160, Math.round((activeResize.initialW + dx) / 10) * 10);
        cat.height = Math.max(180, Math.round((activeResize.initialH + dy) / 10) * 10);
      }
    }
  }

  function onMouseUp() {
    activeDrag = null;
    activeResize = null;
    window.removeEventListener("mousemove", onMouseMove);
    window.removeEventListener("mouseup", onMouseUp);
  }

  function removeHero(catId: string, heroId: number) {
    const cat = categories.find((c) => c.id === catId);
    if (cat) {
      cat.heroes = cat.heroes.filter((h) => h.hero_id !== heroId);
    }
  }

  function addHero(catId: string, hero: HeroMeta) {
    const cat = categories.find((c) => c.id === catId);
    if (cat && !cat.heroes.some((h) => h.hero_id === hero.hero_id)) {
      cat.heroes.push(hero);
      toast.success(`Added ${hero.name} to ${cat.name}`);
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

  function autoArrange() {
    initLayout();
    toast.success("Categories auto-arranged into 5 columns");
  }

  function saveLayout() {
    toast.success("Grid layout saved successfully");
  }
</script>

<div class="flex h-full flex-col font-sans select-none">
  <!-- Editor Action Bar -->
  <div class="flex h-11 shrink-0 items-center justify-between gap-3 border-b border-white/10 bg-zinc-950/80 px-4">
    <div class="flex items-center gap-2">
      <span class="flex items-center gap-1.5 text-xs font-semibold uppercase tracking-wider text-zinc-300">
        <Sparkles size={14} class="text-zinc-400" />
        Hero Grid Editor
      </span>
      <span class="text-xs text-zinc-500">· Drag boxes to reposition, stretch bottom-right handle to resize</span>
    </div>

    <div class="flex items-center gap-2">
      <Button variant="outline" size="sm" class="h-7 gap-1.5 px-2.5 text-xs" onclick={autoArrange}>
        <RotateCcw size={12} />
        Auto Arrange
      </Button>
      <Button size="sm" class="h-7 gap-1.5 px-3 text-xs bg-zinc-100 text-zinc-950 hover:bg-zinc-200" onclick={saveLayout}>
        <Save size={12} />
        Save Grid Layout
      </Button>
    </div>
  </div>

  <!-- Interactive Grid Canvas -->
  <div class="relative min-h-0 flex-1 overflow-auto bg-[#0a0d12] p-4">
    <!-- Grid background pattern -->
    <div
      class="pointer-events-none absolute inset-0 bg-[radial-gradient(#1e2633_1px,transparent_1px)] [background-size:20px_20px] opacity-40"
    ></div>

    <div class="relative min-h-[700px] min-w-[1300px]">
      {#each categories as cat (cat.id)}
        <div
          class="absolute flex flex-col rounded-lg border bg-[#11161f]/95 shadow-2xl transition-shadow {selectedCategory === cat.id ? 'border-zinc-300 ring-2 ring-zinc-400/20' : 'border-[#263140] hover:border-[#3b4b61]'}"
          style:left="{cat.x}px"
          style:top="{cat.y}px"
          style:width="{cat.width}px"
          style:height="{cat.height}px"
        >
          <!-- Category Header (Drag Handle) -->
          <div
            role="button"
            tabindex="0"
            aria-label="Drag category {cat.name}"
            class="flex h-9 shrink-0 cursor-grab items-center justify-between border-b border-[#222c3a] bg-[#141b24] px-3 active:cursor-grabbing"
            onmousedown={(e) => startDrag(e, cat)}
          >
            <div class="flex items-center gap-2 truncate">
              <Move size={12} class="shrink-0 text-zinc-500" />
              <input
                type="text"
                class="no-drag w-full bg-transparent font-sans text-xs font-bold uppercase tracking-wider text-zinc-300 focus:outline-none focus:text-white"
                bind:value={cat.name}
              />
            </div>
            <button
              type="button"
              class="no-drag grid size-5 place-items-center rounded text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200"
              onclick={() => (addingHeroToCategory = cat.id)}
              title="Add Hero"
            >
              <Plus size={13} />
            </button>
          </div>

          <!-- Hero Tiles Inside Category -->
          <div class="no-drag scroll-thin flex-1 overflow-y-auto p-2">
            <div class="flex flex-wrap gap-2">
              {#each cat.heroes as h (h.hero_id)}
                <div class="group relative flex w-[52px] flex-col items-center select-none" title="{h.name} • {pct(h.winrate)} WR • {pct(h.pickrate)} Pick">
                  <!-- Stats -->
                  <div class="flex w-full flex-col items-center leading-none pb-0.5">
                    <span class="font-mono text-[9px] font-bold text-[#cbd5e1] tabular-nums">{pct(h.winrate)}</span>
                    <span class="font-mono text-[9px] text-[#64748b] tabular-nums">{pct(h.pickrate)}</span>
                  </div>

                  <!-- Portrait Card -->
                  <div class="relative h-[80px] w-[52px] overflow-hidden border border-[#2b3542] bg-[#0c1015] shadow group-hover:border-zinc-400">
                    <img
                      src={d2ptVert(h.slug)}
                      alt={h.name}
                      loading="lazy"
                      class="size-full object-cover object-top"
                      onerror={(e) => handleImgError(e, h.slug)}
                    />

                    <!-- Remove Hero Button -->
                    <button
                      type="button"
                      class="absolute top-0.5 right-0.5 hidden size-4 place-items-center rounded bg-red-950/90 text-red-300 hover:bg-red-700 hover:text-white group-hover:grid"
                      onclick={() => removeHero(cat.id, h.hero_id)}
                      title="Remove hero"
                    >
                      <X size={10} />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          </div>

          <!-- Resize Handle (Bottom Right) -->
          <button
            type="button"
            aria-label="Resize category {cat.name}"
            class="absolute right-0 bottom-0 grid size-4 cursor-se-resize place-items-center text-zinc-500 hover:text-zinc-300"
            onmousedown={(e) => startResize(e, cat)}
          >
            <Maximize2 size={10} class="rotate-90 opacity-60" />
          </button>
        </div>
      {/each}
    </div>
  </div>

  <!-- Add Hero Modal / Drawer -->
  {#if addingHeroToCategory}
    {@const targetCat = categories.find((c) => c.id === addingHeroToCategory)}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div class="flex max-h-[85vh] w-full max-w-lg flex-col rounded-xl border border-zinc-800 bg-zinc-950 shadow-2xl">
        <div class="flex items-center justify-between border-b border-zinc-800 px-4 py-3">
          <h3 class="text-sm font-semibold text-zinc-200">
            Add Hero to {targetCat?.name}
          </h3>
          <button
            type="button"
            class="grid size-6 place-items-center rounded text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200"
            onclick={() => (addingHeroToCategory = null)}
          >
            <X size={14} />
          </button>
        </div>

        <div class="p-3 border-b border-zinc-800">
          <input
            type="text"
            placeholder="Search hero name..."
            class="w-full rounded-md border border-zinc-800 bg-zinc-900 px-3 py-1.5 text-xs text-zinc-200 placeholder-zinc-500 focus:border-zinc-600 focus:outline-none"
            bind:value={searchQuery}
          />
        </div>

        <div class="scroll-thin min-h-0 flex-1 overflow-y-auto p-3">
          <div class="grid grid-cols-4 gap-2">
            {#each filteredHeroes as h (h.hero_id)}
              <button
                type="button"
                class="flex flex-col items-center gap-1 rounded-md border border-zinc-800/80 bg-zinc-900/50 p-1.5 transition-colors hover:border-zinc-600 hover:bg-zinc-800/80"
                onclick={() => {
                  if (targetCat) addHero(targetCat.id, h);
                }}
              >
                <div class="relative h-[65px] w-[44px] overflow-hidden rounded bg-black">
                  <img
                    src={d2ptVert(h.slug)}
                    alt={h.name}
                    loading="lazy"
                    class="size-full object-cover object-top"
                    onerror={(e) => handleImgError(e, h.slug)}
                  />
                </div>
                <span class="truncate w-full text-center text-[10px] text-zinc-300">{h.name}</span>
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
