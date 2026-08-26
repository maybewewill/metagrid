<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset=".github/logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset=".github/logo-light.svg">
    <img alt="MetaGrid" src=".github/logo-light.svg" width="96">
  </picture>

  <h1>MetaGrid</h1>
  <p>Automated Dota 2 hero grids synchronized with high-MMR meta statistics.</p>
</div>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%2B-0078D6.svg?style=flat-square&logo=windows)](https://www.microsoft.com/windows)
[![Tauri: v2](https://img.shields.io/badge/Tauri-v2-FFC131.svg?style=flat-square&logo=tauri)](https://v2.tauri.app)
[![Svelte: 5](https://img.shields.io/badge/Svelte-5-FF3E00.svg?style=flat-square&logo=svelte)](https://svelte.dev)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-DEA584.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-3178C6.svg?style=flat-square&logo=typescript)](https://www.typescriptlang.org)

</div>

<div align="center">
  <a href="#overview">Overview</a> &middot;
  <a href="#screenshots">Screenshots</a> &middot;
  <a href="#features">Features</a> &middot;
  <a href="#installation">Installation</a> &middot;
  <a href="#how-it-works">How It Works</a> &middot;
  <a href="#development">Development</a>
</div>

---

## Overview

MetaGrid is a Windows desktop application and background tray utility for Dota 2 players. It fetches meta data directly from [dota2protracker.com](https://dota2protracker.com) (7000+ MMR matches) and generates up-to-date in-game hero grids for all 5 positions (Carry, Mid, Offlane, Support, Hard Support).

Grids are written directly to Dota's `hero_grid_config.json`. Existing user-created grids are preserved without modifications, and automatic backups are created before changes are written.

---

## Screenshots

<div align="center">

### Meta Dashboard (List View)
<img src="docs/images/dashboard.png" alt="MetaGrid Dashboard List View" width="850" />

<br/><br/>

### In-Game Grid Preview
<img src="docs/images/grid-preview.png" alt="MetaGrid In-Game Grid Preview" width="850" />

<br/><br/>

### Settings
<img src="docs/images/settings.png" alt="MetaGrid Settings" width="850" />

</div>

---

## Features

- **Live Meta Fetching**: Parses current patch meta from Dota2ProTracker with winrate, pickrate, and D2PT ratings.
- **In-Game Grid Sync**: Generates 5-column and per-role grids formatted for the Dota 2 picking phase layout.
- **Process Watcher**: Detects when `dota2.exe` starts and automatically refreshes grid files.
- **Multi-Account Discovery**: Detects all local Steam user directories under `Steam/userdata/<id>/570/remote/cfg/`.
- **Atomic File Operations**: Safe writing mechanism using temporary files with `.metagrid.bak` fallback.
- **Fullscreen Adaptive Scaling**: Clean proportional UI scaling for high-resolution displays.
- **Tray & Startup Integration**: Runs minimized in the background with customizable autostart on system boot.
- **Localization**: Full interface support for English and Russian.

---

## Installation

1. Download the latest installer (`MetaGrid_x.y.z_x64-setup.exe`) from [Releases](https://github.com/maybewewill/metagrid/releases).
2. Run the installer and launch **MetaGrid**.
3. Select your language and target Steam account, then click **Fetch & Patch**.
4. In Dota 2, open **Heroes → Hero Grids** and select the **MetaGrid** layout.

---

## How It Works

```mermaid
flowchart LR
    A[Dota2ProTracker] -->|Fetch via Schannel| B(MetaGrid Core)
    B -->|Parse & Rank Heroes| C{Grid Multi-Builder}
    C -->|Top Picks + Meta Pool| D[hero_grid_config.json]
    D -->|Steam Userdata Sync| E[Dota 2 Client]
    F[Dota 2 Process Watcher] -.->|On Game Launch| B
    G[Tray / Scheduler] -.->|Trigger Refresh| B
```

Grid configuration path:
```
<Steam_Installation_Path>/userdata/<Account_ID>/570/remote/cfg/hero_grid_config.json
```

---

## Configuration

| Setting | Options | Description |
| :--- | :--- | :--- |
| **Role Labels** | `Named` / `POS 1-5` | Toggle role naming convention across the UI and in-game grid tabs. |
| **Refresh Interval** | `1h`, `3h`, `6h`, `12h`, `24h` | Background scheduler sync interval. |
| **Steam Account** | `All accounts` or specific Steam ID | Target account selection. |
| **Start with Windows** | `Enabled` / `Disabled` | Launch minimized on system startup. |
| **Language** | `English` / `Russian` | UI language and grid category names. |

---

## Development

### Prerequisites
- Node.js 20+
- Rust 1.85+ stable (with MSVC toolchain)

### Commands

```bash
# Install dependencies
npm install

# Start development mode
npm run tauri dev

# Run frontend tests
npm test

# Run type checks
npm run check

# Run Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Build release installer
npm run tauri build
```

---

## License

Distributed under the [MIT License](LICENSE).
