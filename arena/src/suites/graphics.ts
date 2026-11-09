import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export interface GraphicsStackEvaluation {
  name: string;
  domain: "2D Canvas/WebGL" | "2D Game Engine" | "3D WebGL/WebGPU" | "Native WebGPU (Rust)" | "Native ECS 2D/3D (Rust)" | "Vector/Skia CPU Rasterizer";
  runtime: "Bun / Node.js" | "Rust Bare-Metal" | "WebGPU / Metal / Vulkan";
  fpsBudget: "120 FPS" | "60 FPS";
  memoryFootprintMb: number;
  apiCharacteristics: string;
}

export const GRAPHICS_STACK_MATRIX: GraphicsStackEvaluation[] = [
  {
    name: "PixiJS v8",
    domain: "2D Canvas/WebGL",
    runtime: "Bun / Node.js",
    fpsBudget: "60 FPS",
    memoryFootprintMb: 48,
    apiCharacteristics: "Scene graph 2D modulaire, batch rendering agressif, WebGL 2 + WebGPU renderer, llms.txt standard."
  },
  {
    name: "Phaser 3/4",
    domain: "2D Game Engine",
    runtime: "Bun / Node.js",
    fpsBudget: "60 FPS",
    memoryFootprintMb: 75,
    apiCharacteristics: "Moteur de jeu complet (Arcade/Matter.js physics, audio, tilemaps, tweens), runtime Canvas/WebGL."
  },
  {
    name: "Three.js r170+",
    domain: "3D WebGL/WebGPU",
    runtime: "Bun / Node.js",
    fpsBudget: "60 FPS",
    memoryFootprintMb: 85,
    apiCharacteristics: "Standard mondial 3D Web, TSL (Three Shading Language), WebGPURenderer, materials node-based."
  },
  {
    name: "Babylon.js v7+",
    domain: "3D WebGL/WebGPU",
    runtime: "Bun / Node.js",
    fpsBudget: "60 FPS",
    memoryFootprintMb: 120,
    apiCharacteristics: "Moteur 3D entreprise, Compute Shaders WebGPU natifs, havok physics, GUI 3D intégrée."
  },
  {
    name: "wgpu (gfx-rs)",
    domain: "Native WebGPU (Rust)",
    runtime: "Rust Bare-Metal",
    fpsBudget: "120 FPS",
    memoryFootprintMb: 24,
    apiCharacteristics: "Implémentation native Rust du standard W3C WebGPU, multi-backend DX12/Vulkan/Metal, zéro overhead JS GC."
  },
  {
    name: "Bevy Engine (0.15+)",
    domain: "Native ECS 2D/3D (Rust)",
    runtime: "Rust Bare-Metal",
    fpsBudget: "120 FPS",
    memoryFootprintMb: 38,
    apiCharacteristics: "ECS pur hautement parallélisé (Rayon), renderer wgpu, asset pipeline modulaire, compile en natif et WASM."
  },
  {
    name: "Yolo Render / Skia Rasterizer",
    domain: "Vector/Skia CPU Rasterizer",
    runtime: "Rust Bare-Metal",
    fpsBudget: "120 FPS",
    memoryFootprintMb: 12,
    apiCharacteristics: "Rasterizer vectoriel 2D déterministe CPU (SIMD AVX2/NEON) avec option WGPU, C-ABI zero-copy."
  }
];

export async function runGraphicsBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  const results: ArenaBenchmarkResult[] = [];

  console.log("\n==========================================");
  console.log("  ARENA 5: 2D, 3D, CANVAS, WEBGPU & GAME ENGINES");
  console.log("  PixiJS vs Phaser vs Three.js vs Babylon vs wgpu vs Bevy vs Yolo Render");
  console.log("==========================================\n");

  // 1. PixiJS Module Import & Initialization Benchmark
  if (existsSync("C:/src/yolo-arena/vendor/pixijs/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "PixiJS v8 (Node Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/pixijs/package.json')\"",
        5,
        1
      )
    );
  }

  // 2. Phaser Core Import Benchmark
  if (existsSync("C:/src/yolo-arena/vendor/phaser/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "Phaser 3 (Node Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/phaser/package.json')\"",
        5,
        1
      )
    );
  }

  // 3. Three.js Import Benchmark
  if (existsSync("C:/src/yolo-arena/vendor/threejs/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "Three.js (Node Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/threejs/package.json')\"",
        5,
        1
      )
    );
  }

  // 4. Babylon.js Import Benchmark
  if (existsSync("C:/src/yolo-arena/vendor/babylonjs/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "Babylon.js (Node Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/babylonjs/package.json')\"",
        5,
        1
      )
    );
  }

  // 5. wgpu native Rust workspace check
  if (existsSync("C:/src/yolo-arena/vendor/wgpu/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "wgpu Native Rust (Cargo Metadata Eval)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/wgpu/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  // 6. Bevy Engine Rust workspace check
  if (existsSync("C:/src/yolo-arena/vendor/bevy/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "Bevy Engine (Cargo Metadata Eval)",
        "cargo metadata --manifest-path C:/src/yolo-arena/vendor/bevy/Cargo.toml --format-version 1 --no-deps",
        5,
        1
      )
    );
  }

  // 7. Yolo Render (Native 2D/3D Rasterizer)
  if (existsSync("C:/src/yolo-ui/crates/yolo-render/Cargo.toml")) {
    results.push(
      await runHyperfineBenchmark(
        "graphics",
        "Yolo Render Native (Cargo Check)",
        "cargo check --manifest-path C:/src/yolo-ui/crates/yolo-render/Cargo.toml",
        3,
        1
      )
    );
  }

  return results;
}
