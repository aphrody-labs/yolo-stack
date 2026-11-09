import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export async function runEnvironmentArena(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 6: CROSS-OS & EXECUTION PLATFORMS");
  console.log("  Win32 Native vs Git Bash (Cygwin) vs Brush (Full Rust Bash) vs Remote VPS Ubuntu vs Remote Arch");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. Win32 Native CMD
  results.push(
    await runHyperfineBenchmark(
      "environments",
      "Win32 Native (cmd.exe)",
      "cmd.exe /c echo win32-ok",
      10,
      2
    )
  );

  // 2. Win32 Bun Shell
  results.push(
    await runHyperfineBenchmark(
      "environments",
      "Win32 Bun Shell",
      "bun -e \"import { $ } from 'bun'; await $`echo bun-ok`;\"",
      10,
      2
    )
  );

  // 3. Git Bash (Cygwin POSIX Subsystem on Windows)
  const gitBash = "C:\\Progra~1\\Git\\bin\\bash.exe";
  results.push(
    await runHyperfineBenchmark(
      "environments",
      "Git Bash (Cygwin POSIX)",
      `${gitBash} -c "echo cygwin-ok"`,
      10,
      2
    )
  );

  // 4. Brush (100% Native Rust POSIX/Bash Engine on Windows)
  results.push(
    await runHyperfineBenchmark(
      "environments",
      "Brush (100% Rust Native Bash)",
      "C:\\Users\\aphro\\.cargo\\bin\\brush.exe -c \"echo rust-bash-ok\"",
      10,
      2
    )
  );

  // 5. WSL (Sous-système Windows pour Linux - État sans distribution)
  results.push({
    suite: "environments",
    name: "WSL2 (Sans distribution configurée)",
    meanMs: 0,
    minMs: 0,
    maxMs: 0,
    samples: 0,
    status: "skipped",
    error: "WSL2 actif mais aucune distribution installée (wsl.exe signale code 1)",
  });

  // 6. Remote SSH Ubuntu VPS (Linux Kernel 7.3, prod node 2116886)
  results.push(
    await runHyperfineBenchmark(
      "environments",
      "Remote SSH VPS (Ubuntu 26.10 / Kernel 7.3)",
      "ssh -o BatchMode=yes -o ConnectTimeout=3 vps echo-vps",
      5,
      1
    )
  );

  // 7. Remote SSH Arch Production Node
  results.push({
    suite: "environments",
    name: "Remote SSH Arch Node (Hostinger KVM prod)",
    meanMs: 0,
    minMs: 0,
    maxMs: 0,
    samples: 0,
    status: "skipped",
    error: "Hôte distant injoignable ou en attente d'initialisation WireGuard (timeout port 22)",
  });

  return results;
}
