# MetaGrid

A minimalist Windows tray app that keeps a **current-meta Dota 2 hero grid**
fresh in the background — without ever touching your existing grids.

It pulls the top heroes per role (POS 1–5) from
[dota2protracker.com](https://dota2protracker.com), builds a hero grid, and
writes a single config named **`MetaGrid`** into Dota's
`hero_grid_config.json`. Every other config you have is preserved byte-for-byte.

- **Rust + Tauri v2** backend, **Svelte 5 + TypeScript + Tailwind v4 + shadcn-svelte** UI
- Lives in the system tray, optional autostart
- Background refresh: on launch, on a timer, when you start Dota, or on demand
- Fully localized **EN / RU**
- Never overwrites or deletes your own grids; writes a `.bak` before first change

## Install

Grab the NSIS installer (`MetaGrid_x.y.z_x64-setup.exe`) from a release, or build
it yourself (see below). Windows 10 1803+ required (uses the built-in `curl.exe`).

## First run

1. Pick a language.
2. Pick which Steam/Dota account to write to (or *all* accounts). Accounts are
   auto-detected under `…/Steam/userdata/<id>/570/remote/cfg/`. If none show up,
   launch Dota once so the folder exists, then reopen MetaGrid.
3. **Fetch first meta** — the Dashboard fills with the current per-role meta.

Open Dota → Hero Grids → pick **MetaGrid**.

## Settings

- **Heroes per role** (5–15) and **sort** (pickrate / winrate)
- **Refresh interval** (1/3/6/12/24 h)
- **Account** target, **autostart**, **layout** (single 5-column grid vs. one grid per role)
- **Language** (EN/RU) — live-updates the UI and the in-game grid category names

## Where the grid is written

`…/Steam/userdata/<account>/570/remote/cfg/hero_grid_config.json`

Safety guarantees:
- Only the config named `MetaGrid` (and, in per-role mode, `MetaGrid POS 1…5`) is
  created/updated. All foreign configs are kept untouched.
- A `hero_grid_config.json.metagrid.bak` is written before the first modification.
- Writes are atomic (temp file + rename); a failed fetch never overwrites a good grid.

## Data source & caveat

Meta data is scraped from dota2protracker.com's public homepage via the Windows
`curl.exe` (its Schannel TLS stack passes Cloudflare where library HTTP clients get
403). Respect d2pt's terms of service; this is for personal use. The default refresh
interval is deliberately conservative.

## Swapping the data source

All meta access goes through the `MetaProvider` trait (`src-tauri/src/provider/`).
Adding another source (e.g. STRATZ, OpenDota) is a new file implementing the trait
plus a new arm in `make_provider` (`src-tauri/src/services.rs`) — no changes anywhere
else.

## Development

```bash
npm install
npm run tauri dev      # run the app against the live backend
npm test               # frontend unit tests (vitest)
npm run build          # build the Svelte SPA to dist/
cargo test -p metagrid --lib --manifest-path src-tauri/Cargo.toml   # Rust unit tests
```

Live/network Rust tests are `#[ignore]`d; run with `-- --ignored`.

## Build the installer

```bash
npm run tauri build    # → src-tauri/target/release/bundle/nsis/*.exe
```
