import { test, expect } from "bun:test";
import { runNativePrecisionBenchmark } from "../src/engine.ts";
import { generateMarkdownReport } from "../src/reporter.ts";

test("engine: native precision benchmark measures command execution accurately", async () => {
  const result = await runNativePrecisionBenchmark(
    "test-suite",
    "Bun Echo",
    "bun -e \"process.exit(0)\"",
    3
  );

  expect(result.status).toBe("success");
  expect(result.samples).toBe(3);
  expect(result.meanMs).toBeGreaterThan(0);
  expect(result.minMs).toBeLessThanOrEqual(result.maxMs);
});

test("reporter: generates valid markdown leaderboard with winner ranking", () => {
  const report = generateMarkdownReport([
    {
      suite: "shells",
      name: "Bun Shell",
      meanMs: 8.5,
      minMs: 7.2,
      maxMs: 9.8,
      samples: 5,
      status: "success",
    },
    {
      suite: "shells",
      name: "PowerShell 7",
      meanMs: 45.2,
      minMs: 40.1,
      maxMs: 52.3,
      samples: 5,
      status: "success",
    },
  ]);

  expect(report).toContain("# YOLO ARENA 2026");
  expect(report).toContain("**#1 (Winner)**");
  expect(report).toContain("Bun Shell");
  expect(report).toContain("PowerShell 7");
});
