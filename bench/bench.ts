// yolo-stack benchmark harness. Bun only.
//
//   bun run bench/bench.ts --target baseline            # smoke: framework-free Bun.serve ceiling
//   bun run bench/bench.ts --target tanstack,dioxus     # planned targets (build + serve + load)
//   bun run bench/bench.ts --all --duration 10 --connections 64 --receipt
//
// Per target it records: build wall time, artifact sizes (raw, gzip, brotli), and for every shared
// route the HTTP requests/second, mean/p50/p99 latency and error count from an in-process fetch load
// generator. Results go to bench/out/<date>-<target>.json and, with --receipt, docs/receipts/.
// The load generator shares the host with the server; numbers are relative between stacks on one
// machine, never absolute. See docs/BENCHMARKS.md for the status of each target.
import { ROUTES } from "@yolo-rs/stack-routes";
import { brotliCompressSync, gzipSync } from "node:zlib";
import { resolve } from "node:path";

interface Target {
  cwd: string;
  build: string[] | null;
  serve: string[];
  port: number;
  artifacts: string[];
  status: "runnable" | "planned";
}

interface LoadResult {
  status: number;
  bytes: number;
  requests: number;
  errors: number;
  seconds: number;
  rps: number;
  mean_ms: number;
  p50_ms: number;
  p99_ms: number;
}

interface RouteResult extends LoadResult {
  id: string;
  path: string;
}

const args = new Map<string, string>();
const argv = process.argv.slice(2);
for (let i = 0; i < argv.length; i++) {
  const a = argv[i]!;
  if (!a.startsWith("--")) continue;
  const next = argv[i + 1];
  if (next && !next.startsWith("--")) {
    args.set(a.slice(2), next);
    i++;
  } else {
    args.set(a.slice(2), "true");
  }
}

const root = resolve(import.meta.dir, "..");
const targetsFile = (await Bun.file(
  resolve(import.meta.dir, "targets.json"),
).json()) as {
  targets: Record<string, Target>;
};
const duration = Number(args.get("duration") ?? 5);
const connections = Number(args.get("connections") ?? 32);
const warmup = Number(args.get("warmup") ?? 1);
const wanted = args.has("all")
  ? Object.keys(targetsFile.targets)
  : (args.get("target") ?? "baseline").split(",");

function percentile(sorted: number[], p: number): number {
  if (sorted.length === 0) return Number.NaN;
  const idx = Math.min(
    sorted.length - 1,
    Math.floor((p / 100) * sorted.length),
  );
  return sorted[idx]!;
}

async function waitForPort(port: number, ms = 30_000): Promise<void> {
  const deadline = Date.now() + ms;
  while (Date.now() < deadline) {
    try {
      await fetch(`http://127.0.0.1:${port}/`);
      return;
    } catch {
      await Bun.sleep(100);
    }
  }
  throw new Error(`port ${port} did not open within ${ms}ms`);
}

async function load(
  url: string,
  seconds: number,
  conns: number,
): Promise<LoadResult> {
  const latencies: number[] = [];
  let requests = 0;
  let errors = 0;
  let status = 0;
  let bytes = 0;
  const end = performance.now() + seconds * 1000;
  async function worker(): Promise<void> {
    while (performance.now() < end) {
      const t0 = performance.now();
      try {
        const res = await fetch(url);
        const body = await res.arrayBuffer();
        status = res.status;
        bytes = body.byteLength;
        if (!res.ok) errors++;
      } catch {
        errors++;
      }
      latencies.push(performance.now() - t0);
      requests++;
    }
  }
  const t0 = performance.now();
  await Promise.all(Array.from({ length: conns }, worker));
  const elapsed = (performance.now() - t0) / 1000;
  latencies.sort((a, b) => a - b);
  const mean =
    latencies.reduce((a, b) => a + b, 0) / Math.max(1, latencies.length);
  return {
    status,
    bytes,
    requests,
    errors,
    seconds: +elapsed.toFixed(3),
    rps: +(requests / elapsed).toFixed(1),
    mean_ms: +mean.toFixed(3),
    p50_ms: +percentile(latencies, 50).toFixed(3),
    p99_ms: +percentile(latencies, 99).toFixed(3),
  };
}

async function artifactSizes(cwd: string, globs: string[]) {
  const files: { file: string; raw: number; gzip: number; brotli: number }[] =
    [];
  for (const g of globs) {
    for await (const f of new Bun.Glob(g).scan({ cwd, absolute: true })) {
      const buf = new Uint8Array(await Bun.file(f).arrayBuffer());
      files.push({
        file: f.replace(root, "").replaceAll("\\", "/"),
        raw: buf.byteLength,
        gzip: gzipSync(buf).byteLength,
        brotli: brotliCompressSync(buf).byteLength,
      });
    }
  }
  const total = files.reduce(
    (a, f) => ({
      raw: a.raw + f.raw,
      gzip: a.gzip + f.gzip,
      brotli: a.brotli + f.brotli,
    }),
    { raw: 0, gzip: 0, brotli: 0 },
  );
  return { files, total };
}

async function runTarget(
  name: string,
  t: Target,
): Promise<Record<string, unknown>> {
  const cwd = resolve(root, t.cwd);
  const report: Record<string, unknown> = {
    target: name,
    status: t.status,
    date: new Date().toISOString(),
    host: {
      platform: process.platform,
      arch: process.arch,
      bun: Bun.version,
      cpus: navigator.hardwareConcurrency,
    },
    settings: { duration, connections, warmup },
    note: "load generator and server share one host; compare targets only within one session",
  };
  if (t.status !== "runnable" && !args.has("force")) {
    report.skipped = `target is '${t.status}'; pass --force once ${t.cwd} exists`;
    return report;
  }
  if (t.build) {
    const t0 = performance.now();
    const p = Bun.spawn(t.build, { cwd, stdout: "inherit", stderr: "inherit" });
    const code = await p.exited;
    report.build = {
      command: t.build.join(" "),
      seconds: +((performance.now() - t0) / 1000).toFixed(2),
      exit: code,
    };
    if (code !== 0) return report;
  }
  report.artifacts = await artifactSizes(cwd, t.artifacts);
  const server = Bun.spawn(t.serve, {
    cwd,
    env: { ...process.env, PORT: String(t.port) },
    stdout: "ignore",
    stderr: "inherit",
  });
  try {
    await waitForPort(t.port);
    const routes: RouteResult[] = [];
    for (const r of ROUTES) {
      const url = `http://127.0.0.1:${t.port}${r.path}`;
      if (warmup > 0) await load(url, warmup, Math.min(8, connections));
      const res = {
        id: r.id,
        path: r.path,
        ...(await load(url, duration, connections)),
      };
      routes.push(res);
      console.log(
        `${name} ${r.id.padEnd(16)} ${String(res.rps).padStart(9)} req/s  p50 ${res.p50_ms} ms  p99 ${res.p99_ms} ms  ${res.bytes} B`,
      );
    }
    report.routes = routes;
  } finally {
    server.kill();
    await server.exited;
  }
  return report;
}

const outDir = resolve(import.meta.dir, "out");
const stamp = new Date().toISOString().slice(0, 10);
for (const name of wanted) {
  const t = targetsFile.targets[name];
  if (!t) {
    console.error(`unknown target ${name}`);
    process.exitCode = 1;
    continue;
  }
  const report = await runTarget(name, t);
  const json = JSON.stringify(report, null, 2);
  await Bun.write(resolve(outDir, `${stamp}-${name}.json`), json);
  if (args.has("receipt")) {
    await Bun.write(
      resolve(root, "docs/receipts", `bench-${name}-${stamp}.json`),
      json,
    );
  }
  if (report.skipped) console.log(`${name}: ${report.skipped}`);
}
