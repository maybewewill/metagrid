import { describe, it, expect } from "vitest";
import { relTime, pct, formatWr, pct2, roleLabel } from "$lib/format";

describe("format", () => {
  const now = new Date("2026-08-26T12:00:00Z");
  it("relTime", () => {
    expect(relTime("2026-08-26T11:59:30Z", now)).toBe("just now");
    expect(relTime("2026-08-26T11:55:00Z", now)).toBe("5 min ago");
    expect(relTime("2026-08-26T10:00:00Z", now)).toBe("2 h ago");
    expect(relTime("2026-08-26T11:59:30Z", now, "ru")).toBe("только что");
    expect(relTime("2026-08-26T11:55:00Z", now, "ru")).toBe("5 мин назад");
    expect(relTime("2026-08-26T10:00:00Z", now, "ru")).toBe("2 ч назад");
  });
  it("pct", () => expect(pct(0.523)).toBe("52%"));
  it("formatWr", () => {
    expect(formatWr(0.523)).toBe("52.3%");
    expect(formatWr(1.0)).toBe("100%");
  });
  it("pct2", () => {
    expect(pct2(0.538)).toBe("53.80%");
    expect(pct2(1.0)).toBe("100%");
  });
  it("roleLabel", () => expect(roleLabel("Pos1")).toBe("role.pos1"));
});
