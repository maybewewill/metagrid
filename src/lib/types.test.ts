import { describe, it, expect } from "vitest";
import { isMetaSnapshot } from "$lib/types";

describe("isMetaSnapshot", () => {
  it("accepts a valid snapshot", () => {
    expect(isMetaSnapshot({ patch: "7.41e", fetched_at: "t", source: "d2pt", roles: [] })).toBe(
      true,
    );
  });
  it("rejects junk", () => expect(isMetaSnapshot({ nope: 1 })).toBe(false));
});
