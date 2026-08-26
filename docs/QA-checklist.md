# MetaGrid — Manual QA Checklist

Native behaviour that unit tests can't cover. Run through once on the real
machine after `npm run tauri dev` (or an installed build) before shipping.

## Tray & window
- [ ] Tray icon appears on launch.
- [ ] Left-click tray → main window shows and focuses.
- [ ] Tray menu: **Refresh now** triggers a refresh (status dot pulses).
- [ ] Tray menu: **Open MetaGrid** shows the window.
- [ ] Tray menu: **Quit** exits the process (tray icon disappears).
- [ ] Closing the window (X) **hides** it — app keeps running in tray, does not quit.
- [ ] Window is frameless; custom titlebar drag moves it; minimize button works.

## Autostart
- [ ] Settings → Autostart ON → `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
      gains a `MetaGrid` entry (check `regedit` or Task Manager → Startup).
- [ ] Autostart OFF → the Run entry is removed.

## Refresh pipeline
- [ ] First run: Onboarding → pick language → pick account → **Fetch first meta**
      completes and lands on the Dashboard with 5 role columns populated.
- [ ] Dashboard shows current patch and “updated Xm ago”.
- [ ] Refresh button re-fetches; status dot goes green (Ok) on success.

## Grid safety (the critical one)
- [ ] After a refresh, open Dota 2 → hero grids: a config named **MetaGrid** exists,
      organised POS 1–5.
- [ ] Every pre-existing config (e.g. *Main Layout*, custom grids) is **still present
      and unchanged**.
- [ ] A `hero_grid_config.json.metagrid.bak` backup was created next to the real file.
- [ ] Layout mode = per-role (Settings): five **MetaGrid POS n** configs appear; foreign
      configs still intact.

## Dota launch watcher
- [ ] Launch Dota 2 (any way) → a Windows notification fires
      (“Meta refreshed for next game”) and the next-session grid is refreshed.
- [ ] **Play Dota** button in the app: refreshes, then Steam launches Dota (`steam://rungameid/570`).

## Failure handling
- [ ] Disconnect network → Refresh → status goes **Error**, a toast shows, and the
      previous grid on disk is **left intact** (no empty/partial write).
- [ ] Reconnect → Refresh recovers to Ok.

## Presentation
- [ ] Hero portraits render (cached under `%APPDATA%/com.metagrid.app/portraits/`);
      missing portraits fall back to initials, never a broken-image icon.
- [ ] Switching language (Settings / Onboarding) live-updates all UI text **and** the
      in-game grid category names on the next write (EN ⇄ RU).
- [ ] OS “reduced motion” setting suppresses hover/stagger animations.
