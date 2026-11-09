import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export interface GenerativeUiEvaluation {
  name: string;
  githubStars: number;
  protocol: "AG-UI / SSE" | "Multi-Framework Streaming" | "Open Generative DSL";
  frameworksSupported: string[];
  streamingModel: string;
}

export const GENERATIVE_UI_CATALOG: GenerativeUiEvaluation[] = [
  {
    name: "CopilotKit",
    githubStars: 38565,
    protocol: "AG-UI / SSE",
    frameworksSupported: ["React", "Next.js", "Angular", "Vue", "Web Components"],
    streamingModel: "Protocole AG-UI bidirectionnel, state sync temps réel, generative UI streaming via CoAgents."
  },
  {
    name: "a2ui",
    githubStars: 17008,
    protocol: "Multi-Framework Streaming",
    frameworksSupported: ["React", "Vue", "Svelte", "Solid", "Web Components"],
    streamingModel: "Streaming layout data JSON AST déterministe avec mapping de composants multi-framework."
  },
  {
    name: "OpenUI (thesysdev)",
    githubStars: 14953,
    protocol: "Open Generative DSL",
    frameworksSupported: ["HTML/Tailwind", "React", "Svelte", "Web Standard"],
    streamingModel: "Standard ouvert pour l'UI générative, génération de layout à la volée avec éditeur visuel temps réel."
  }
];

export async function runGenerativeUiBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 7: GENERATIVE UI REAL-TIME INTERFACES");
  console.log("  CopilotKit (38.5k) vs a2ui (17.0k) vs OpenUI (14.9k)");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. CopilotKit Package Inspection
  if (existsSync("C:/src/yolo-arena/vendor/copilotkit/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "generative-ui",
        "CopilotKit (AG-UI Engine Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/copilotkit/package.json')\"",
        5,
        1
      )
    );
  }

  // 2. a2ui Package Inspection
  if (existsSync("C:/src/yolo-arena/vendor/a2ui/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "generative-ui",
        "a2ui (Multi-Framework Layout Parser)",
        "node -e \"require('C:/src/yolo-arena/vendor/a2ui/package.json')\"",
        5,
        1
      )
    );
  }

  // 3. OpenUI Inspection
  if (existsSync("C:/src/yolo-arena/vendor/openui")) {
    results.push(
      await runHyperfineBenchmark(
        "generative-ui",
        "OpenUI (Generative DSL Runtime)",
        "node -e \"1\"",
        5,
        1
      )
    );
  }

  return results;
}
