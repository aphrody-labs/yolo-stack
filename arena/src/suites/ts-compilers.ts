import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync, writeFileSync } from "fs";
import { ARENA_ROOT } from "../paths.ts";

export interface TsCompilerEvaluation {
  name: string;
  languageBackend: "Rust" | "Go" | "Zig" | "TypeScript";
  typeChecking: boolean;
  transpilationModel: string;
}

export const TS_COMPILERS_CATALOG: TsCompilerEvaluation[] = [
  {
    name: "oxc / oxlint",
    languageBackend: "Rust",
    typeChecking: false,
    transpilationModel: "Suite d'outillage JS/TS en Rust pur la plus rapide au monde (ast, parser, linter, minifier)."
  },
  {
    name: "esbuild",
    languageBackend: "Go",
    transpilationModel: "Compilateur et bundler Go pionnier du parallélisme extrême sans overhead de mémoire.",
    typeChecking: false
  },
  {
    name: "swc",
    languageBackend: "Rust",
    transpilationModel: "Compilateur extensible en Rust (utilisé par Next.js/Turbopack, Vite, Deno).",
    typeChecking: false
  },
  {
    name: "bun (Transpiler + AOT compiler)",
    languageBackend: "Rust",
    transpilationModel: "Transpileur TypeScript instantané écrit en Rust pur (architecture monorepo upstream Cargo avec intégration JavaScriptCore/C++) et compilation AOT en binaire natif.",
    typeChecking: false
  },
  {
    name: "tsc (TypeScript Official)",
    languageBackend: "TypeScript",
    typeChecking: true,
    transpilationModel: "Compilateur de référence officiel de Microsoft fournissant la vérification statique complète des types."
  }
];

export async function runTsCompilersBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 10: TYPESCRIPT & JS COMPILERS & TRANSPILERS");
  console.log("  oxc/oxlint (Rust) vs esbuild (Go) vs swc (Rust) vs bun (Zig) vs tsc (TS)");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // Création d'un fichier source TS représentatif pour le benchmark
  const testTsFile = `${ARENA_ROOT}/results/benchmark_target.ts`;
  writeFileSync(testTsFile, `
export interface User {
  id: string;
  name: string;
  roles: string[];
}
export function processUser(user: User): string {
  return \`User: \${user.name} (\${user.roles.join(', ')})\`;
}
export const users: User[] = Array.from({ length: 1000 }, (_, i) => ({
  id: \`id_\${i}\`,
  name: \`User \${i}\`,
  roles: ['admin', 'developer']
}));
`);

  // 1. Bun Transpiler / Runner
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "Bun (Rust Native TS Transpiler)",
      `bun build ${testTsFile} --outfile ${ARENA_ROOT}/results/out-bun.js`,
      10,
      2
    )
  );

  // 2. Bun AOT Compile (Single executable binaire natif)
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "Bun AOT Compile (Standalone Executable)",
      `bun build ${testTsFile} --compile --outfile ${ARENA_ROOT}/results/out-bun.exe`,
      5,
      1
    )
  );

  // 3. esbuild (Go Native)
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "esbuild (Go Native Bundler/Compiler)",
      `esbuild ${testTsFile} --outfile=${ARENA_ROOT}/results/out-esbuild.js`,
      10,
      2
    )
  );

  // 4. oxlint (Rust Oxc Engine)
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "oxlint (Rust Oxc AST & Linter)",
      `oxlint ${testTsFile}`,
      10,
      2
    )
  );

  // 5. swc (Rust Native)
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "swc (Rust Compiler)",
      `swc ${testTsFile} -o ${ARENA_ROOT}/results/out-swc.js`,
      10,
      2
    )
  );

  // 6. tsc (Microsoft Official TypeScript)
  results.push(
    await runHyperfineBenchmark(
      "ts-compilers",
      "tsc (Microsoft Official Type Checker)",
      `tsc --noEmit ${testTsFile}`,
      5,
      1
    )
  );

  return results;
}
