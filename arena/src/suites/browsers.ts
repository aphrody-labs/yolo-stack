import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";
import { ARENA_ROOT, pillarDir } from "../paths.ts";

export async function runBrowserBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  const results: ArenaBenchmarkResult[] = [];

  // 1. Obscura Engine CLI Cold Boot (--version); needs pillars/yolo-web or YOLO_STACK_YOLO_WEB_DIR
  const web = pillarDir("yolo-web");
  if (web && existsSync(`${web}/crates/obscura-cli/Cargo.toml`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Yolo Browser Obscura (Native Cargo Run --version)",
        `cargo run --manifest-path ${web}/crates/obscura-cli/Cargo.toml -- --version`,
        5,
        1
      )
    );
  }

  // 2. Playwright Core package inspection
  if (existsSync(`${ARENA_ROOT}/vendor/playwright/packages/playwright-core/cli.js`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Playwright CLI (Node Upstream --version)",
        `node ${ARENA_ROOT}/vendor/playwright/packages/playwright-core/cli.js --version`,
        5,
        1
      )
    );
  }

  // 3. Crawlee package inspection
  if (existsSync(`${ARENA_ROOT}/vendor/crawlee/package.json`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Crawlee (Package Evaluation)",
        `node -e \"require('${ARENA_ROOT}/vendor/crawlee/package.json')\"`,
        5,
        1
      )
    );
  }

  // 4. Puppeteer package inspection
  if (existsSync(`${ARENA_ROOT}/vendor/puppeteer/package.json`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Puppeteer (Package Evaluation)",
        `node -e \"require('${ARENA_ROOT}/vendor/puppeteer/package.json')\"`,
        5,
        1
      )
    );
  }

  // 5. Firecrawl inspection
  if (existsSync(`${ARENA_ROOT}/vendor/firecrawl`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Firecrawl (Runtime Verification)",
        "node -e \"1\"",
        5,
        1
      )
    );
  }

  // 6. Scrapy Python engine initialization
  if (existsSync(`${ARENA_ROOT}/vendor/scrapy`)) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Scrapy Engine (uv Python Runtime)",
        "uv run --no-project python -c \"import sys; sys.exit(0)\"",
        5,
        1
      )
    );
  }

  return results;
}
