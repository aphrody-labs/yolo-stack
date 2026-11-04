import { describe, expect, test } from "bun:test";
import {
  MIN_BUN_VERSION,
  assertBunVersion,
  bunVersion,
  compareVersions,
  parseVersion,
  satisfiesMinimum,
} from "./src/index.ts";

describe("parseVersion", () => {
  test("analyse une version simple", () => {
    expect(parseVersion("1.4.4")).toEqual({ major: 1, minor: 4, patch: 4 });
  });

  test("tolere le prefixe v et les suffixes", () => {
    expect(parseVersion("v2.0.1-canary.3")).toEqual({ major: 2, minor: 0, patch: 1 });
  });

  test("rejette une chaine invalide", () => {
    expect(() => parseVersion("abc")).toThrow(TypeError);
  });
});

describe("compareVersions", () => {
  test("ordonne correctement", () => {
    expect(compareVersions(parseVersion("1.4.4"), parseVersion("1.4.4"))).toBe(0);
    expect(compareVersions(parseVersion("1.4.3"), parseVersion("1.4.4"))).toBeLessThan(0);
    expect(compareVersions(parseVersion("1.5.0"), parseVersion("1.4.9"))).toBeGreaterThan(0);
    expect(compareVersions(parseVersion("2.0.0"), parseVersion("1.99.99"))).toBeGreaterThan(0);
  });
});

describe("runtime", () => {
  test("Bun.version est exposee et satisfait le pin", () => {
    expect(bunVersion()).toBe(Bun.version);
    expect(satisfiesMinimum(bunVersion(), MIN_BUN_VERSION)).toBe(true);
    expect(() => assertBunVersion()).not.toThrow();
  });

  test("un minimum futur echoue", () => {
    expect(() => assertBunVersion("999.0.0")).toThrow(/999\.0\.0/);
  });
});
