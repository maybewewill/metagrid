import type { Position } from "$lib/types";

export function relTime(iso: string, now: Date = new Date()): string {
  const then = new Date(iso).getTime();
  const diffMs = now.getTime() - then;
  const diffMin = Math.floor(diffMs / 60000);

  if (diffMin < 1) return "just now";
  if (diffMin < 60) return `${diffMin} min ago`;

  const diffHours = Math.floor(diffMin / 60);
  return `${diffHours} h ago`;
}

export function pct(x: number): string {
  return `${Math.round(x * 100)}%`;
}

export function pct2(x: number): string {
  return `${(x * 100).toFixed(2)}%`;
}

export function roleLabel(position: Position): string {
  return `role.${position.toLowerCase()}`;
}
