import { existsSync } from "node:fs";
import { resolve } from "node:path";

const posix = (path: string): string => path.replaceAll("\\", "/");

/** Root of the arena package (this repository's `arena/` directory), never a hardcoded drive. */
export const ARENA_ROOT = posix(resolve(import.meta.dir, ".."));

/**
 * Directory of a pillar. `YOLO_STACK_<NAME>_DIR` (name upper-cased, `-` as `_`) overrides the
 * merged copy under `pillars/<name>`; returns `undefined` when neither exists so the benchmark is skipped.
 */
export function pillarDir(name: string): string | undefined {
  const env = process.env[`YOLO_STACK_${name.toUpperCase().replaceAll("-", "_")}_DIR`];
  const dir = env ? resolve(env) : resolve(ARENA_ROOT, "..", "pillars", name);
  return existsSync(dir) ? posix(dir) : undefined;
}
