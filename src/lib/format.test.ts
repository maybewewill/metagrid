import { describe, it, expect } from "vitest";
import { relTime, pct, roleLabel } from "$lib/format";

describe("format", () => {
  const now = new Date("2026-08-26T12:00:00Z");
  it("relTime", () => {
    expect(relTime("2026-08-26T11:59:30Z", now)).toBe("just now");
    expect(relTime("2026-08-26T11:55:00Z", now)).toBe("5 min ago");
    expect(relTime("2026-08-26T10:00:00Z", now)).toBe("2 h ago");
  });
  it("pct", () => expect(pct(0.523)).toBe("52%"));
  it("roleLabel", () => expect(roleLabel("Pos1")).toBe("role.pos1"));
});
