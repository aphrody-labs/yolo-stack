import { runShellArena } from "./suites/shells.ts";
import { runRuntimeArena } from "./suites/runtimes.ts";
import { runGuiArena } from "./suites/gui.ts";
import { runBrowserBenchmarks } from "./suites/browsers.ts";
import { runGraphicsBenchmarks } from "./suites/graphics.ts";
import { runEnvironmentArena } from "./suites/environments.ts";
import { runGenerativeUiBenchmarks } from "./suites/generative-ui.ts";
import { runGameAgentsBenchmarks } from "./suites/game-agents.ts";
import { runMcpOfficialBenchmarks } from "./suites/mcp-protocol.ts";
import { runTsCompilersBenchmarks } from "./suites/ts-compilers.ts";
import { runFullStackRustBenchmarks } from "./suites/fullstack-rust.ts";
import { generateMarkdownReport } from "./reporter.ts";
import { existsSync, mkdirSync } from "fs";
import { ARENA_ROOT } from "./paths.ts";

async function main() {
  const args = process.argv.slice(2);
  const suiteArg = args.find((a, i) => args[i - 1] === "--suite" || a.startsWith("--suite="))?.split("=")[1] || "all";

  console.log(`[yolo-arena] Initializing Arena Engine 2026. Target suite: ${suiteArg}`);
  if (!existsSync(`${ARENA_ROOT}/results`)) {
    mkdirSync(`${ARENA_ROOT}/results`, { recursive: true });
  }

  const allResults = [];

  if (suiteArg === "all" || suiteArg === "shells") {
    console.log("[yolo-arena] Running Shells Arena (Bun Shell vs CMD vs PowerShell vs NuShell vs Bash)...");
    allResults.push(...await runShellArena());
  }

  if (suiteArg === "all" || suiteArg === "runtimes") {
    console.log("[yolo-arena] Running Runtimes Arena (Bun vs Node vs Deno)...");
    allResults.push(...await runRuntimeArena());
  }

  if (suiteArg === "all" || suiteArg === "gui") {
    console.log("[yolo-arena] Running GUI / Layout Arena (GPUI vs egui vs iced vs tauri vs taffy vs electron)...");
    allResults.push(...await runGuiArena());
  }

  if (suiteArg === "all" || suiteArg === "browsers") {
    console.log("[yolo-arena] Running Browsers Arena (Yolo Obscura vs Playwright vs Puppeteer vs Crawlee)...");
    allResults.push(...await runBrowserBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "graphics") {
    console.log("[yolo-arena] Running Graphics & Game Engines Arena (PixiJS vs Phaser vs Three.js vs Babylon vs wgpu vs Bevy vs Yolo Render)...");
    allResults.push(...await runGraphicsBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "environments") {
    console.log("[yolo-arena] Running Cross-OS & Platform Environments Arena (Win32 vs Git Bash vs Brush Rust Bash vs Remote VPS Ubuntu)...");
    allResults.push(...await runEnvironmentArena());
  }

  if (suiteArg === "all" || suiteArg === "generative-ui") {
    console.log("[yolo-arena] Running Generative UI Arena (CopilotKit vs a2ui vs OpenUI)...");
    allResults.push(...await runGenerativeUiBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "game-agents") {
    console.log("[yolo-arena] Running Game Playing LLM Agents Arena (Voyager vs AI_Diplomacy vs civ6-mcp)...");
    allResults.push(...await runGameAgentsBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "mcp-protocol") {
    console.log("[yolo-arena] Running MCP Official Protocols Arena (servers vs typescript-sdk vs python-sdk vs inspector)...");
    allResults.push(...await runMcpOfficialBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "ts-compilers") {
    console.log("[yolo-arena] Running TypeScript & JS Compilers Arena (oxc vs esbuild vs swc vs bun vs bun aot vs tsc)...");
    allResults.push(...await runTsCompilersBenchmarks());
  }

  if (suiteArg === "all" || suiteArg === "fullstack-rust") {
    console.log("[yolo-arena] Running Full-Stack Rust Frameworks Arena (Dioxus vs Leptos vs Tauri vs Loco)...");
    allResults.push(...await runFullStackRustBenchmarks());
  }

  const report = generateMarkdownReport(allResults);
  const reportPath = `${ARENA_ROOT}/ARENA_REPORT_2026.md`;
  await Bun.write(reportPath, report);

  console.log(`[yolo-arena] Scientific benchmark report successfully written to ${reportPath}`);
}

main().catch((err) => {
  console.error("[yolo-arena] Fatal error during benchmark run:", err);
  process.exit(1);
});
