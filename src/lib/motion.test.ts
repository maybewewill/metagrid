import { describe, it, expect, vi } from "vitest";
import { prefersReducedMotion } from "$lib/motion";
describe("motion", () =>
  it("respects reduced-motion match", () => {
    vi.stubGlobal("matchMedia", (q: string) => ({ matches: true, media: q, addEventListener() {}, removeEventListener() {} }));
    expect(prefersReducedMotion()).toBe(true);
  }));
