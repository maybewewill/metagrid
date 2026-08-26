// TS mirrors of the Rust domain types in src-tauri/src/model.rs,
// settings.rs, and state.rs. Keep these in lockstep with the backend —
// they are the contract for the typed IPC layer (Task 4.3).

export type Position = "Pos1" | "Pos2" | "Pos3" | "Pos4" | "Pos5";

export type SortMetric = "Pickrate" | "Winrate";

export interface HeroMeta {
  hero_id: number;
  name: string;
  slug: string;
  winrate: number;
  pickrate: number;
  matches: number;
}

export interface RoleMeta {
  position: Position;
  role_winrate: number;
  heroes: HeroMeta[];
}

export interface MetaSnapshot {
  patch: string;
  fetched_at: string;
  source: string;
  roles: RoleMeta[];
}

export interface Settings {
  top_n: number;
  sort: SortMetric;
  interval_hours: number;
  account_id: string | null;
  autostart: boolean;
  layout_columns: boolean;
  lang: "en" | "ru";
  onboarded: boolean;
}

export interface Account {
  id: string;
}

// Mirrors state.rs `Status` with #[serde(tag = "kind", content = "detail")].
export type Status =
  | { kind: "Idle" }
  | { kind: "Refreshing" }
  | { kind: "Ok" }
  | { kind: "Stale" }
  | { kind: "Error"; detail: string };

export function isMetaSnapshot(x: unknown): x is MetaSnapshot {
  if (typeof x !== "object" || x === null) return false;
  const s = x as Record<string, unknown>;
  return (
    typeof s.patch === "string" &&
    typeof s.fetched_at === "string" &&
    typeof s.source === "string" &&
    Array.isArray(s.roles)
  );
}
