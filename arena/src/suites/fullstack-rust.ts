import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export interface FullStackRustEvaluation {
  name: string;
  targetPlatforms: ("Web" | "Desktop" | "Mobile" | "CLI" | "Server")[];
  reactivityModel: "Fine-Grained Signals" | "Virtual DOM / Retained Scene" | "Rails-like MVC ActiveModel" | "Native IPC Webview";
  description: string;
}

export const FULLSTACK_RUST_CATALOG: FullStackRustEvaluation[] = [
  {
    name: "Dioxus",
    targetPlatforms: ["Web", "Desktop", "Mobile", "CLI", "Server"],
    reactivityModel: "Virtual DOM / Retained Scene",
    description: "Framework fullstack universel en Rust pur: compile en WebAssembly, desktop natif (Wry/Tao), iOS, Android et SSR."
  },
  {
    name: "Leptos",
    targetPlatforms: ["Web", "Server"],
    reactivityModel: "Fine-Grained Signals",
    description: "Framework fullstack SSR ultra-réactif inspiré de SolidJS avec signaux fins sans Virtual DOM."
  },
  {
    name: "Tauri v2",
    targetPlatforms: ["Desktop", "Mobile"],
    reactivityModel: "Native IPC Webview",
    description: "Plateforme applicative cross-plateforme utilisant les webviews système (WebView2, WebKit) et un backend Rust léger."
  },
  {
    name: "Loco.rs",
    targetPlatforms: ["Server", "CLI"],
    reactivityModel: "Rails-like MVC ActiveModel",
    description: "Le Ruby on Rails de Rust: backend monolithique productif avec SeaORM, Axum et génération de code intégrée."
  }
];

export async function runFullStackRustBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 11: FULL-STACK RUST FRAMEWORKS");
  console.log("  Dioxus vs Leptos vs Tauri vs Loco.rs");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. Dioxus Workspace Evaluation
  if (existsSync("C:/src/yolo-arena/vendor/dioxus/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "fullstack-rust",
        "Dioxus Fullstack Workspace (Cargo Metadata)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/dioxus/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  // 2. Leptos Workspace Evaluation
  if (existsSync("C:/src/yolo-arena/vendor/leptos/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "fullstack-rust",
        "Leptos Reactive SSR Workspace (Cargo Metadata)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/leptos/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  // 3. Tauri v2 Workspace Evaluation
  if (existsSync("C:/src/yolo-arena/vendor/tauri/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "fullstack-rust",
        "Tauri v2 Core Workspace (Cargo Metadata)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/tauri/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  // 4. Loco.rs Framework Evaluation
  if (existsSync("C:/src/yolo-arena/vendor/loco/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "fullstack-rust",
        "Loco.rs MVC Framework (Cargo Metadata)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/loco/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  return results;
}
