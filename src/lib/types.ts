export type Position = "Pos1" | "Pos2" | "Pos3" | "Pos4" | "Pos5";

export type SortMetric = "Pickrate" | "Winrate";

export type MetaMode = "matches" | "matches_wr" | "d2ptrating";

export interface HeroMeta {
  hero_id: number;
  name: string;
  slug: string;
  winrate: number;
  pickrate: number;
  matches: number;
  d2pt_rating?: number;
  is_top?: boolean;
}

export interface RoleMeta {
  position: Position;
  role_winrate: number;
  heroes: HeroMeta[];
}

export interface Category {
  category_name: string;
  x_position: number;
  y_position: number;
  width: number;
  height: number;
  hero_ids: number[];
}

export interface GridConfig {
  config_name: string;
  categories: Category[];
}

export interface MetaSnapshot {
  patch: string;
  fetched_at: string;
  source: string;
  roles: RoleMeta[];
  configs?: GridConfig[];
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
  grid_mode: "separate" | "merge";
  merge_target: string | null;
  meta_mode: MetaMode;
}

export interface Account {
  id: string;
}

export interface Tournament {
  id: number;
  name: string;
  match_count: number;
}

export type Status =
  | { kind: "Idle" }
  | { kind: "Refreshing" }
  | { kind: "Ok" }
  | { kind: "Stale" }
  | { kind: "Error"; detail: string };

export interface UpdateInfo {
  available: boolean;
  current_version: string;
  latest_version: string;
  release_url: string;
  release_notes?: string;
  download_url?: string;
}

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
