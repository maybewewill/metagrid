<script lang="ts">
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { Sparkles, Download, AlertCircle, RefreshCw, X } from "@lucide/svelte";
  import { store } from "$lib/store.svelte";
  import * as ipc from "$lib/ipc";
  import { Button } from "$lib/components/ui/button";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let installing = $state(false);
  let progress = $state(0);
  let errorMsg = $state<string | null>(null);
  let isLaunching = $state(false);

  onMount(() => {
    let unlisten: UnlistenFn | undefined;
    ipc.onUpdateProgress((pct) => {
      progress = Math.round(pct);
      if (pct >= 100) {
        isLaunching = true;
      }
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  });

  async function startUpdate() {
    installing = true;
    errorMsg = null;
    progress = 0;
    isLaunching = false;

    try {
      await ipc.installUpdate(store.updateInfo?.download_url);
    } catch (e) {
      errorMsg = String(e);
      installing = false;
    }
  }

  function close() {
    if (!installing) {
      store.showUpdateModal = false;
    }
  }
</script>

{#if store.showUpdateModal && store.updateInfo?.available}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-md transition-opacity"
      onclick={close}
      role="presentation"
    ></div>

    <div
      class="relative z-10 flex w-full max-w-[460px] flex-col gap-4 rounded-xl border border-white/10 bg-[#0e1219]/95 p-5 shadow-2xl backdrop-blur-xl"
      role="dialog"
      aria-modal="true"
      aria-labelledby="update-modal-title"
    >
      <div class="flex items-start justify-between gap-3">
        <div class="flex items-center gap-3">
          <div class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-emerald-500/30 bg-emerald-500/10 text-emerald-400 shadow-[0_0_12px_rgba(16,185,129,0.2)]">
            <Sparkles size={18} />
          </div>
          <div class="flex flex-col">
            <h2 id="update-modal-title" class="text-sm font-bold tracking-tight text-white">
              {$_("update_modal.title")}
            </h2>
            <span class="text-xs text-muted-foreground">
              {$_("update_modal.subtitle")}
            </span>
          </div>
        </div>

        {#if !installing}
          <button
            type="button"
            class="flex size-7 items-center justify-center rounded-md text-zinc-400 hover:bg-white/10 hover:text-white transition-colors"
            onclick={close}
            aria-label="Close"
          >
            <X size={15} />
          </button>
        {/if}
      </div>

      <div class="flex items-center justify-between rounded-lg border border-white/5 bg-black/30 px-3.5 py-2 text-xs">
        <span class="text-zinc-400">Version</span>
        <span class="font-mono font-semibold text-emerald-400">
          {$_("update_modal.current_vs_latest", {
            values: {
              current: store.appVersion.replace(/^v/, ''),
              latest: store.updateInfo.latest_version
            }
          })}
        </span>
      </div>

      {#if store.updateInfo.release_notes}
        <div class="flex flex-col gap-1.5">
          <span class="text-[11px] font-semibold uppercase tracking-wider text-zinc-400">
            {$_("update_modal.whats_new")}
          </span>
          <div class="max-h-40 overflow-y-auto rounded-lg border border-white/5 bg-black/40 p-3 text-xs leading-relaxed text-zinc-300 whitespace-pre-wrap select-text font-sans">
            {store.updateInfo.release_notes}
          </div>
        </div>
      {/if}

      {#if errorMsg}
        <div class="flex items-center gap-2 rounded-lg border border-rose-500/30 bg-rose-950/30 p-3 text-xs text-rose-300">
          <AlertCircle size={15} class="shrink-0 text-rose-400" />
          <span class="flex-1">{$_("update_modal.failed", { values: { error: errorMsg } })}</span>
        </div>
      {/if}

      {#if installing}
        <div class="flex flex-col gap-2 py-1">
          <div class="flex items-center justify-between text-xs font-medium">
            <span class="text-zinc-300">
              {isLaunching ? $_("update_modal.launching") : $_("update_modal.downloading")}
            </span>
            <span class="font-mono text-emerald-400">{progress}%</span>
          </div>
          <div class="h-2 w-full overflow-hidden rounded-full bg-zinc-800/80 border border-white/5">
            <div
              class="h-full bg-emerald-500 transition-all duration-150 ease-out"
              style="width: {progress}%"
            ></div>
          </div>
        </div>
      {/if}

      <div class="flex items-center justify-end gap-2 pt-1">
        {#if !installing}
          <Button
            variant="ghost"
            size="sm"
            class="rounded-sm text-xs text-zinc-400 hover:text-white"
            onclick={close}
          >
            {$_("update_modal.later")}
          </Button>

          <Button
            size="sm"
            class="gap-1.5 rounded-sm bg-emerald-500 text-zinc-950 font-bold hover:bg-emerald-400 text-xs shadow-sm"
            onclick={startUpdate}
          >
            <Download size={13} />
            <span>{errorMsg ? $_("update_modal.retry") : $_("update_modal.update_now")}</span>
          </Button>
        {:else}
          <Button
            size="sm"
            disabled
            class="gap-1.5 rounded-sm bg-emerald-500/70 text-zinc-950 font-bold text-xs"
          >
            <RefreshCw size={13} class="animate-spin" />
            <span>{isLaunching ? $_("update_modal.launching") : $_("update_modal.downloading")}</span>
          </Button>
        {/if}
      </div>
    </div>
  </div>
{/if}
