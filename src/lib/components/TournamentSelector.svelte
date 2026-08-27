<script lang="ts">
  import { _ } from "svelte-i18n";
  import { store } from "$lib/store.svelte";
  import type { Tournament } from "$lib/types";
  import fallbackTournaments from "$lib/tournaments.json";
  import { Trophy, Search, ChevronDown, Check, X } from "@lucide/svelte";

  const tournaments = $derived<Tournament[]>(
    store.tournaments.length > 0 ? store.tournaments : (fallbackTournaments as Tournament[])
  );

  let open = $state(false);
  let search = $state("");
  let containerEl = $state<HTMLDivElement | null>(null);

  const currentId = $derived(store.settings?.league_id ?? -1);
  const currentTournament = $derived(
    tournaments.find((t) => t.id === currentId) ?? tournaments[0] ?? { id: -1, name: "All Tournaments", match_count: 0 }
  );

  const filtered = $derived.by(() => {
    const q = search.trim().toLowerCase();
    if (!q) return tournaments;
    return tournaments.filter((t) => t.name.toLowerCase().includes(q));
  });

  function toggleOpen() {
    open = !open;
    if (open) {
      store.fetchTournaments().catch(() => {});
    }
  }

  async function handleSelect(t: Tournament) {
    if (t.id === currentId) {
      open = false;
      return;
    }
    open = false;
    search = "";
    await store.saveSettings({ league_id: t.id });
    await store.fetchOnly();
  }

  function handleWindowClick(e: MouseEvent) {
    if (open && containerEl && !containerEl.contains(e.target as Node)) {
      open = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && open) {
      open = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleKeydown} />

<div bind:this={containerEl} class="relative inline-block text-left">
  <button
    type="button"
    class="inline-flex h-8 items-center justify-center gap-2 rounded-sm border border-border bg-background px-3 text-xs font-semibold text-foreground shadow-xs transition-colors hover:bg-muted hover:text-foreground"
    onclick={toggleOpen}
    aria-expanded={open}
  >
    <Trophy size={14} class="text-muted-foreground shrink-0" />
    <span class="max-w-[200px] truncate text-center text-xs font-medium translate-y-[1px]">{currentTournament.name}</span>
    <ChevronDown size={13} class="text-muted-foreground shrink-0 transition-transform duration-200 {open ? 'rotate-180' : ''}" />
  </button>

  {#if open}
    <div
      class="absolute left-0 top-full z-50 mt-1.5 w-72 origin-top-left rounded-sm border border-border bg-popover p-1.5 text-popover-foreground shadow-xl ring-1 ring-black/30"
    >
      <div class="relative mb-1 flex items-center border-b border-border px-2 pb-1.5 pt-0.5">
        <Search size={13} class="text-muted-foreground shrink-0 mr-1.5" />
        <input
          type="text"
          placeholder={$_("tournaments.search")}
          bind:value={search}
          class="w-full bg-transparent text-xs text-foreground placeholder-muted-foreground focus:outline-none"
        />
        {#if search}
          <button type="button" onclick={() => (search = "")} class="text-muted-foreground hover:text-foreground">
            <X size={13} />
          </button>
        {/if}
      </div>

      <div class="scroll-thin max-h-60 overflow-y-auto space-y-0.5">
        {#each filtered as t (t.id)}
          {@const isSelected = t.id === currentId}
          <button
            type="button"
            class="flex w-full items-center justify-between gap-2 rounded-xs px-2 py-1.5 text-left text-xs transition-colors {isSelected ? 'bg-secondary font-semibold text-secondary-foreground' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
            onclick={() => handleSelect(t)}
          >
            <div class="flex min-w-0 items-center gap-1.5">
              <span class="truncate">{t.name}</span>
            </div>
            <div class="flex items-center gap-1.5 shrink-0">
              {#if t.match_count > 0}
                <span class="rounded-xs bg-muted px-1 py-0.5 text-[10px] font-mono text-muted-foreground">
                  {t.match_count}
                </span>
              {/if}
              {#if isSelected}
                <Check size={13} class="text-foreground" />
              {/if}
            </div>
          </button>
        {/each}

        {#if filtered.length === 0}
          <div class="px-3 py-4 text-center text-xs text-muted-foreground">
            {$_("tournaments.no_results")}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
