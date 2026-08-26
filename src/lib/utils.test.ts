import { describe, it, expect } from "vitest";
import { cn } from "$lib/utils";

describe("cn", () => it("merges + dedupes", () => expect(cn("p-2", "p-4")).toBe("p-4")));
