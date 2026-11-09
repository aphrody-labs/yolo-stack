import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";

export async function runRuntimeArena(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 2: JS/TS RUNTIMES & V8 ENGINES");
  console.log("  Bun Upstream vs Node.js Upstream vs Deno V8");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. Bun Upstream
  console.log("[Arena:Runtimes] Testing Bun Upstream 1.4.4+...");
  results.push(
    await runHyperfineBenchmark(
      "runtimes",
      "bun_upstream",
      "bun -e 1",
      10,
      3
    )
  );

  // 2. Node.js Upstream
  console.log("[Arena:Runtimes] Testing Node.js 26.13.0 LTS...");
  results.push(
    await runHyperfineBenchmark(
      "runtimes",
      "node_upstream",
      "node -e 1",
      10,
      3
    )
  );

  // 3. Deno V8
  console.log("[Arena:Runtimes] Testing Deno 2.9.8...");
  results.push(
    await runHyperfineBenchmark(
      "runtimes",
      "deno_v8",
      "deno eval 1",
      10,
      3
    )
  );

  return results;
}
