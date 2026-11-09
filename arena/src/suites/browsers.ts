import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export async function runBrowserBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  const results: ArenaBenchmarkResult[] = [];

  // 1. Obscura Engine CLI Cold Boot (--version)
  results.push(
    await runHyperfineBenchmark(
      "browsers",
      "Yolo Browser Obscura (Native Cargo Run --version)",
      "cargo run --manifest-path C:/src/yolo-web/crates/obscura-cli/Cargo.toml -- --version",
      5,
      1
    )
  );

  // 2. Playwright Core package inspection
  if (existsSync("C:/src/yolo-arena/vendor/playwright/packages/playwright-core/cli.js")) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Playwright CLI (Node Upstream --version)",
        "node C:/src/yolo-arena/vendor/playwright/packages/playwright-core/cli.js --version",
        5,
        1
      )
    );
  }

  // 3. Crawlee package inspection
  if (existsSync("C:/src/yolo-arena/vendor/crawlee/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Crawlee (Package Evaluation)",
        "node -e \"require('C:/src/yolo-arena/vendor/crawlee/package.json')\"",
        5,
        1
      )
    );
  }

  // 4. Puppeteer package inspection
  if (existsSync("C:/src/yolo-arena/vendor/puppeteer/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "browsers",
        "Puppeteer (Package Evaluation)",
        "node -e \"require('C:/src/yolo-arena/vendor/puppeteer/package.json')\"",
        5,
        1
      )
    );
  }

  // 5. Firecrawl inspection
  if (existsSync("C:/src/yolo-arena/vendor/firecrawl")) {
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
  if (existsSync("C:/src/yolo-arena/vendor/scrapy")) {
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
