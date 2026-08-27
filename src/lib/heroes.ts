import heroMapData from "./heroMap.json";

export interface HeroEntry {
  id: number;
  name: string;
  slug: string;
}

const map: Record<number, HeroEntry> = heroMapData as Record<number, HeroEntry>;

export function getHeroById(id: number): HeroEntry {
  return map[id] || { id, name: `Hero ${id}`, slug: "" };
}
