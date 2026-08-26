import type { Position } from "$lib/types";

/** Human-readable relative time, e.g. "5 min ago", "2 h ago", "just now". */
export function relTime(iso: string, now: Date = new Date()): string {
  const then = new Date(iso).getTime();
  const diffMs = now.getTime() - then;
  const diffMin = Math.floor(diffMs / 60000);

  if (diffMin < 1) return "just now";
  if (diffMin < 60) return `${diffMin} min ago`;

  const diffHours = Math.floor(diffMin / 60);
  return `${diffHours} h ago`;
}

/** Format a 0..1 fraction as a rounded whole-number percentage, e.g. 0.523 -> "52%". */
export function pct(x: number): string {
  return `${Math.round(x * 100)}%`;
}

/** Format a 0..1 fraction as a 2-decimal percentage, e.g. 0.538 -> "53.80%". */
export function pct2(x: number): string {
  return `${(x * 100).toFixed(2)}%`;
}

/**
 * Returns the i18n key for a role position (e.g. "Pos1" -> "role.pos1").
 * Callers resolve the localized label via `$_(roleLabel(position))`.
 */
export function roleLabel(position: Position): string {
  return `role.${position.toLowerCase()}`;
}
