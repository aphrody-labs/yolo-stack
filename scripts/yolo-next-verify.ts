#!/usr/bin/env bun
// SPDX-License-Identifier: Apache-2.0
// scripts/yolo-next-verify.ts: runs the yolo-next quality gates (the same ones as .github/workflows/ci.yml,
// scoped to the crate and the package) and writes a dated receipt under docs/receipts/.
//
//   bun scripts/yolo-next-verify.ts [--no-receipt] [--skip-clippy] [--skip-bun] [--fixture <dir>]

import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "..");
const PACKAGE_DIR = join(ROOT, "packages", "yolo-next");

interface StepResult {
  readonly name: string;
  readonly command: string;
  readonly cwd: string;
  readonly exitCode: number;
  readonly durationMs: number;
  readonly tail: string;
}

async function run(name: string, command: string[], cwd: string, env?: Record<string, string>): Promise<StepResult> {
  const started = performance.now();
  const proc = Bun.spawn(command, {
    cwd,
    env: { ...Bun.env, ...env },
    stdout: "pipe",
    stderr: "pipe",
  });
  const [out, err, exitCode] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  const combined = `${out}\n${err}`.trim();
  const lines = combined.split(/\r?\n/);
  const tail = lines.slice(-25).join("\n");
  const result: StepResult = {
    name,
    command: command.join(" "),
    cwd,
    exitCode,
    durationMs: Math.round(performance.now() - started),
    tail,
  };
  console.log(`[${exitCode === 0 ? "ok" : "FAIL"}] ${name} (${result.durationMs} ms)`);
  if (exitCode !== 0) console.log(tail);
  return result;
}

function flag(name: string): boolean {
  return Bun.argv.includes(name);
}

function option(name: string): string | undefined {
  const index = Bun.argv.indexOf(name);
  return index === -1 ? undefined : Bun.argv[index + 1];
}

const fixture = option("--fixture") ?? join(ROOT, "apps", "next-fixture");
const steps: StepResult[] = [];

steps.push(await run("cargo fmt --check (yolo-next)", ["cargo", "fmt", "-p", "yolo-next", "--check"], ROOT));
if (!flag("--skip-clippy")) {
  steps.push(
    await run(
      "cargo clippy -D warnings (yolo-next)",
      ["cargo", "clippy", "-p", "yolo-next", "--all-targets", "--", "-D", "warnings"],
      ROOT,
    ),
  );
}
steps.push(await run("cargo test (yolo-next)", ["cargo", "test", "-p", "yolo-next"], ROOT));
steps.push(
  await run(
    "yolo-next analyze fixture",
    ["cargo", "run", "-q", "-p", "yolo-next", "--", "analyze", fixture],
    ROOT,
  ),
);
if (!flag("--skip-bun")) {
  steps.push(await run("bun test (@yolo-rs/next)", ["bun", "test"], PACKAGE_DIR));
  steps.push(await run("bun check (@yolo-rs/next)", ["bun", "check"], PACKAGE_DIR));
  steps.push(await run("bun run typecheck (root tsc)", ["bun", "run", "typecheck"], ROOT));
}

const ok = steps.every((s) => s.exitCode === 0);
const receipt = {
  tool: "scripts/yolo-next-verify.ts",
  date: new Date().toISOString(),
  host: { platform: process.platform, arch: process.arch, bun: Bun.version },
  versions: {
    cargo: (await run("cargo --version", ["cargo", "--version"], ROOT)).tail,
    rustc: (await run("rustc --version", ["rustc", "--version"], ROOT)).tail,
  },
  fixture,
  ok,
  steps,
};

if (!flag("--no-receipt")) {
  const stamp = receipt.date.slice(0, 10);
  const path = join(ROOT, "docs", "receipts", `yolo-next-${stamp}.json`);
  await Bun.write(path, `${JSON.stringify(receipt, null, 2)}\n`);
  console.log(`receipt: ${path}`);
}

console.log(ok ? "yolo-next: all gates passed" : "yolo-next: gates FAILED");
process.exit(ok ? 0 : 1);
