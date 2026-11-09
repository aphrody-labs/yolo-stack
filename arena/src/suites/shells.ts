import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine";

export async function runShellArena(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 1: SHELLS EXECUTION BENCHMARK");
  console.log("  PowerShell vs CMD vs Bun Shell vs NuShell vs Bash");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. Bun Shell (built-in, zero-process spawning overhead for scripts)
  console.log("[Arena:Shells] Testing Bun Shell...");
  const bunShellRes = await runHyperfineBenchmark(
    "shells",
    "bun_shell",
    `bun -e "import { $ } from 'bun'; await $\`echo test\`;"`,
    10,
    3
  );
  results.push(bunShellRes);

  // 2. CMD (Windows Native Command Interpreter)
  console.log("[Arena:Shells] Testing CMD...");
  const cmdRes = await runHyperfineBenchmark(
    "shells",
    "cmd",
    "cmd.exe /c echo test",
    10,
    3
  );
  results.push(cmdRes);

  // 3. PowerShell (pwsh)
  console.log("[Arena:Shells] Testing PowerShell Core (pwsh)...");
  const pwshRes = await runHyperfineBenchmark(
    "shells",
    "pwsh",
    "pwsh.exe -NoProfile -NonInteractive -Command Write-Output test",
    10,
    2
  );
  results.push(pwshRes);

  // 4. NuShell (if available)
  const nuPath = Bun.which("nu");
  if (nuPath) {
    console.log("[Arena:Shells] Testing NuShell (nu)...");
    const nuRes = await runHyperfineBenchmark(
      "shells",
      "nushell",
      `${nuPath} -c "echo test"`,
      10,
      2
    );
    results.push(nuRes);
  } else {
    results.push({
      suite: "shells",
      name: "nushell",
      meanMs: 0,
      minMs: 0,
      maxMs: 0,
      samples: 0,
      status: "skipped",
      error: "NuShell binary not installed in PATH",
    });
  }

  // 5. Bash (Git Bash / MSYS2 / WSL if available)
  const bashPath = Bun.which("bash");
  if (bashPath) {
    console.log("[Arena:Shells] Testing Bash...");
    const bashRes = await runHyperfineBenchmark(
      "shells",
      "bash",
      `${bashPath} -c "echo test"`,
      10,
      2
    );
    results.push(bashRes);
  }

  return results;
}
