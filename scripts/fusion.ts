#!/usr/bin/env bun
// SPDX-License-Identifier: Apache-2.0
// scripts/fusion.ts: mounts, inspects, and merges the YOLO fleet declared in fusion.toml.
//
//   bun fusion status [name...]      git state of every declared repository (read-only)
//   bun fusion link [name...]        junction/symlink repos/<name> -> local checkout (gitignored)
//   bun fusion unlink [name...]      remove links created by `link`
//   bun fusion plan                  Cargo members and Bun workspaces that fusion would add
//   bun fusion submodule <name...>   replace a link by a pinned git submodule
//   bun fusion subtree <name...>     merge the repository history under repos/<name> (final fusion)
//   bun fusion doctor                verify manifest, mounts, and toolchain

import { existsSync, lstatSync, readlinkSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";

export interface FusionRepo {
  readonly name: string;
  /** Optional sibling checkout, resolved from the environment by `siblingDir`; empty when absent. */
  readonly local: string;
  readonly remote: string;
  readonly role: string;
  readonly kind: string;
  readonly license: string;
  readonly cargo_members: readonly string[];
  readonly bun_workspaces: readonly string[];
}

export interface FusionManifest {
  readonly fusion: {
    mount_dir: string;
    default_mode: "link" | "submodule" | "subtree";
    org: string;
  };
  readonly repo: readonly FusionRepo[];
}

export const STACK_ROOT = resolve(import.meta.dir, "..");

/** Sibling checkout of a fleet repository: `YOLO_STACK_<NAME>_DIR`, else `YOLO_STACK_SIBLINGS_DIR/<name>`, else "" (feature skipped). */
export function siblingDir(name: string): string {
  const env = process.env[`YOLO_STACK_${name.toUpperCase().replaceAll("-", "_")}_DIR`];
  if (env) return resolve(env);
  const base = process.env.YOLO_STACK_SIBLINGS_DIR;
  return base ? resolve(base, name) : "";
}

export async function loadFusionManifest(
  root: string = STACK_ROOT,
): Promise<FusionManifest> {
  const text = await Bun.file(join(root, "fusion.toml")).text();
  const parsed = Bun.TOML.parse(text) as FusionManifest;
  if (!parsed.fusion || !Array.isArray(parsed.repo)) {
    throw new Error("fusion.toml: expected [fusion] table and [[repo]] entries");
  }
  return {
    ...parsed,
    repo: parsed.repo.map((r) => ({ ...r, local: r.local || siblingDir(r.name) })),
  };
}

export type MountState = "missing" | "link" | "directory" | "submodule";

function isReparsePoint(path: string): boolean {
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink()) return true;
  } catch {
    return false;
  }
  // Windows directory junctions are reported as plain directories by some
  // lstat implementations; readlink succeeds on a junction and throws otherwise.
  try {
    readlinkSync(path);
    return true;
  } catch {
    return false;
  }
}

export function mountState(
  root: string,
  manifest: FusionManifest,
  name: string,
): MountState {
  const mount = join(root, manifest.fusion.mount_dir, name);
  if (!existsSync(mount)) return "missing";
  if (isReparsePoint(mount)) return "link";
  const gitEntry = join(mount, ".git");
  if (existsSync(gitEntry) && lstatSync(gitEntry).isFile()) return "submodule";
  return "directory";
}

async function git(cwd: string, ...args: string[]): Promise<string> {
  const proc = Bun.spawn(["git", ...args], {
    cwd,
    stdout: "pipe",
    stderr: "pipe",
  });
  const [out, err, code] = await Promise.all([
    new Response(proc.stdout).text(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  if (code !== 0) {
    throw new Error(`git ${args.join(" ")} failed (${code}): ${err.trim()}`);
  }
  return out.trim();
}

export interface RepoStatus {
  readonly name: string;
  readonly local: string;
  readonly exists: boolean;
  readonly branch?: string;
  readonly head?: string;
  readonly dirty?: number;
  readonly mount: MountState;
}

export async function fusionStatus(
  names: readonly string[] = [],
  root: string = STACK_ROOT,
): Promise<RepoStatus[]> {
  const manifest = await loadFusionManifest(root);
  const selected = manifest.repo.filter(
    (r) => names.length === 0 || names.includes(r.name),
  );
  const out: RepoStatus[] = [];
  for (const repo of selected) {
    const exists = repo.local !== "" && existsSync(repo.local);
    const base: RepoStatus = {
      name: repo.name,
      local: repo.local,
      exists,
      mount: mountState(root, manifest, repo.name),
    };
    if (exists && existsSync(join(repo.local, ".git"))) {
      try {
        const branch = await git(repo.local, "rev-parse", "--abbrev-ref", "HEAD");
        const head = await git(repo.local, "rev-parse", "--short", "HEAD");
        const porcelain = await git(repo.local, "status", "--porcelain");
        out.push({
          ...base,
          branch,
          head,
          dirty: porcelain ? porcelain.split("\n").length : 0,
        });
        continue;
      } catch {
        // fall through with filesystem-only facts
      }
    }
    out.push(base);
  }
  return out;
}

async function createLink(target: string, mount: string): Promise<void> {
  if (process.platform === "win32") {
    // Directory junctions need no privileges and are followed by cargo, bun, and git.
    const proc = Bun.spawn(
      ["cmd", "/c", "mklink", "/J", mount, target.replace(/\//g, "\\")],
      { stdout: "pipe", stderr: "pipe" },
    );
    const code = await proc.exited;
    if (code !== 0) {
      const err = await new Response(proc.stderr).text();
      throw new Error(`mklink /J failed for ${mount}: ${err}`);
    }
    return;
  }
  const { symlink } = await import("node:fs/promises");
  await symlink(target, mount, "dir");
}

export async function fusionLink(
  names: readonly string[] = [],
  root: string = STACK_ROOT,
): Promise<string[]> {
  const manifest = await loadFusionManifest(root);
  const mountDir = join(root, manifest.fusion.mount_dir);
  const linked: string[] = [];
  for (const repo of manifest.repo) {
    if (names.length > 0 && !names.includes(repo.name)) continue;
    if (!repo.local || !existsSync(repo.local)) continue;
    if (mountState(root, manifest, repo.name) !== "missing") continue;
    await createLink(repo.local, join(mountDir, repo.name));
    linked.push(repo.name);
  }
  return linked;
}

export async function fusionUnlink(
  names: readonly string[] = [],
  root: string = STACK_ROOT,
): Promise<string[]> {
  const manifest = await loadFusionManifest(root);
  const removed: string[] = [];
  for (const repo of manifest.repo) {
    if (names.length > 0 && !names.includes(repo.name)) continue;
    const mount = join(root, manifest.fusion.mount_dir, repo.name);
    if (!existsSync(mount)) continue;
    // Only reparse points are removed; a real directory (submodule or subtree) is kept.
    if (isReparsePoint(mount)) {
      rmSync(mount, { recursive: false, force: true });
      removed.push(repo.name);
    }
  }
  return removed;
}

export interface FusionPlan {
  readonly cargoMembers: string[];
  readonly bunWorkspaces: string[];
  readonly repos: { name: string; mode: string; present: boolean }[];
}

export async function fusionPlan(root: string = STACK_ROOT): Promise<FusionPlan> {
  const manifest = await loadFusionManifest(root);
  const cargoMembers: string[] = [];
  const bunWorkspaces: string[] = [];
  const repos: FusionPlan["repos"] = [];
  for (const repo of manifest.repo) {
    repos.push({
      name: repo.name,
      mode: manifest.fusion.default_mode,
      present: repo.local !== "" && existsSync(repo.local),
    });
    for (const member of repo.cargo_members) {
      cargoMembers.push(`${manifest.fusion.mount_dir}/${repo.name}/${member}`);
    }
    for (const ws of repo.bun_workspaces) {
      bunWorkspaces.push(`${manifest.fusion.mount_dir}/${repo.name}/${ws}`);
    }
  }
  return { cargoMembers, bunWorkspaces, repos };
}

export async function fusionSubmodule(
  names: readonly string[],
  root: string = STACK_ROOT,
): Promise<string[]> {
  const manifest = await loadFusionManifest(root);
  const done: string[] = [];
  for (const repo of manifest.repo) {
    if (!names.includes(repo.name)) continue;
    const source = repo.remote || repo.local;
    if (!source) continue;
    await fusionUnlink([repo.name], root);
    await git(
      root,
      "submodule",
      "add",
      "--depth",
      "1",
      source,
      `${manifest.fusion.mount_dir}/${repo.name}`,
    );
    done.push(repo.name);
  }
  return done;
}

export async function fusionSubtree(
  names: readonly string[],
  root: string = STACK_ROOT,
): Promise<string[]> {
  const manifest = await loadFusionManifest(root);
  const done: string[] = [];
  for (const repo of manifest.repo) {
    if (!names.includes(repo.name)) continue;
    const localPresent = repo.local.length > 0 && existsSync(repo.local);
    const source = localPresent ? repo.local : repo.remote;
    if (!source) continue;
    await fusionUnlink([repo.name], root);
    const branch = localPresent
      ? await git(repo.local, "rev-parse", "--abbrev-ref", "HEAD")
      : "main";
    await git(
      root,
      "subtree",
      "add",
      `--prefix=${manifest.fusion.mount_dir}/${repo.name}`,
      source,
      branch,
      "--squash",
    );
    done.push(repo.name);
  }
  return done;
}

export async function fusionDoctor(
  root: string = STACK_ROOT,
): Promise<{ ok: boolean; findings: string[] }> {
  const findings: string[] = [];
  const manifest = await loadFusionManifest(root);
  const names = new Set<string>();
  for (const repo of manifest.repo) {
    if (names.has(repo.name)) findings.push(`duplicate repo name: ${repo.name}`);
    names.add(repo.name);
    if (repo.local && !existsSync(repo.local)) {
      findings.push(`${repo.name}: local checkout not found at ${repo.local}`);
    }
  }
  if (!Bun.version.startsWith("1.4")) {
    findings.push(`bun ${Bun.version} found, 1.4.x expected`);
  }
  return { ok: findings.length === 0, findings };
}

function printTable(rows: RepoStatus[]): void {
  const header = [
    "repo".padEnd(16),
    "local".padEnd(8),
    "branch".padEnd(10),
    "head".padEnd(10),
    " dirty",
    "mount",
  ].join("  ");
  const lines = rows.map((r) =>
    [
      r.name.padEnd(16),
      (r.exists ? "present" : "absent").padEnd(8),
      (r.branch ?? "-").padEnd(10),
      (r.head ?? "-").padEnd(10),
      String(r.dirty ?? "-").padStart(6),
      r.mount,
    ].join("  "),
  );
  console.log([header, ...lines].join("\n"));
}

if (import.meta.main) {
  const [command = "status", ...names] = Bun.argv.slice(2);
  switch (command) {
    case "status":
      printTable(await fusionStatus(names));
      break;
    case "link":
      console.log(JSON.stringify({ linked: await fusionLink(names) }));
      break;
    case "unlink":
      console.log(JSON.stringify({ removed: await fusionUnlink(names) }));
      break;
    case "plan":
      console.log(JSON.stringify(await fusionPlan(), null, 2));
      break;
    case "submodule":
      console.log(JSON.stringify({ submodules: await fusionSubmodule(names) }));
      break;
    case "subtree":
      console.log(JSON.stringify({ subtrees: await fusionSubtree(names) }));
      break;
    case "doctor": {
      const report = await fusionDoctor();
      console.log(JSON.stringify(report, null, 2));
      process.exit(report.ok ? 0 : 1);
      break;
    }
    default:
      console.error(
        `unknown command ${command}; expected status|link|unlink|plan|submodule|subtree|doctor`,
      );
      process.exit(2);
  }
}
