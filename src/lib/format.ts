import type { Position } from "$lib/types";

export function relTime(iso: string, now: Date = new Date(), lang: string = "en"): string {
  const then = new Date(iso).getTime();
  const diffMs = now.getTime() - then;
  const diffMin = Math.floor(diffMs / 60000);

  if (lang === "ru") {
    if (diffMin < 1) return "только что";
    if (diffMin < 60) return `${diffMin} мин назад`;
    const diffHours = Math.floor(diffMin / 60);
    return `${diffHours} ч назад`;
  }

  if (diffMin < 1) return "just now";
  if (diffMin < 60) return `${diffMin} min ago`;

  const diffHours = Math.floor(diffMin / 60);
  return `${diffHours} h ago`;
}

export function pct(x: number): string {
  return `${Math.round(x * 100)}%`;
}

export function formatWr(x: number): string {
  const val = x * 100;
  if (Math.abs(val - 100) < 1e-4) return "100%";
  if (Math.abs(val - 0) < 1e-4) return "0%";
  return `${val.toFixed(1)}%`;
}

export function pct2(x: number): string {
  const val = x * 100;
  if (Math.abs(val - 100) < 1e-4) return "100%";
  if (Math.abs(val - 0) < 1e-4) return "0%";
  return `${val.toFixed(2)}%`;
}

export function roleLabel(position: Position): string {
  return `role.${position.toLowerCase()}`;
}
