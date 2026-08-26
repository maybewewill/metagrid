export type Position = "Pos1" | "Pos2" | "Pos3" | "Pos4" | "Pos5";

export type SortMetric = "Pickrate" | "Winrate";

export type RoleLabelStyle = "named" | "pos";

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
  role_labels: RoleLabelStyle;
}

export interface Account {
  id: string;
}

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
