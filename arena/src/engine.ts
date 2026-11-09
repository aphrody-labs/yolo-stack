import { $ } from "bun";
import { existsSync } from "fs";

export interface ArenaBenchmarkResult {
  suite: string;
  name: string;
  meanMs: number;
  minMs: number;
  maxMs: number;
  memoryMb?: number;
  p95Ms?: number;
  samples: number;
  status: "success" | "skipped" | "failed";
  error?: string;
}

/**
 * Runs a command using `hyperfine` with JSON export for reproducible, scientific micro-benchmarking.
 */
export async function runHyperfineBenchmark(
  suite: string,
  name: string,
  command: string,
  runs = 10,
  warmup = 2
): Promise<ArenaBenchmarkResult> {
  const jsonPath = `results/${suite}-${name.replace(/[^a-zA-Z0-9_-]/g, "_")}.json`;
  try {
    const proc = Bun.spawn([
      "hyperfine",
      "-S",
      "--warmup",
      warmup.toString(),
      "--runs",
      runs.toString(),
      "--export-json",
      jsonPath,
      command,
    ], {
      cwd: "C:/src/yolo-arena",
      stdout: "pipe",
      stderr: "pipe",
    });

    const exitCode = await proc.exited;
    if (exitCode === 0 && existsSync(`C:/src/yolo-arena/${jsonPath}`)) {
      const data = await Bun.file(`C:/src/yolo-arena/${jsonPath}`).json();
      const res = data.results[0];
      return {
        suite,
        name,
        meanMs: res.mean * 1000,
        minMs: res.min * 1000,
        maxMs: res.max * 1000,
        samples: res.times.length,
        status: "success",
      };
    }
  } catch (err: any) {
    // Fallback to high-precision timer in Bun
  }

  // Fallback high-resolution Bun benchmark runner
  return runNativePrecisionBenchmark(suite, name, command, runs);
}

/**
 * Native in-process precision benchmarker.
 */
export async function runNativePrecisionBenchmark(
  suite: string,
  name: string,
  command: string,
  runs = 5
): Promise<ArenaBenchmarkResult> {
  const times: number[] = [];
  try {
    for (let i = 0; i < runs; i++) {
      const t0 = performance.now();
      const proc = Bun.spawn(command.split(" "), {
        stdout: "ignore",
        stderr: "ignore",
      });
      await proc.exited;
      times.push(performance.now() - t0);
    }
    const sum = times.reduce((a, b) => a + b, 0);
    return {
      suite,
      name,
      meanMs: sum / times.length,
      minMs: Math.min(...times),
      maxMs: Math.max(...times),
      samples: times.length,
      status: "success",
    };
  } catch (e: any) {
    return {
      suite,
      name,
      meanMs: 0,
      minMs: 0,
      maxMs: 0,
      samples: 0,
      status: "failed",
      error: String(e),
    };
  }
}
