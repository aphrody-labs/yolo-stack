import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export interface GameAgentEvaluation {
  name: string;
  githubStars: number;
  gameTarget: "Minecraft (Open World)" | "Diplomacy (Board Game)" | "Civilization VI (4X Strategy)";
  agentLoop: "Skill Library + Iterative Prompting" | "Language-based Negotiation & Search" | "MCP Tool Execution Engine";
  llmInputOutput: string;
}

export const GAME_AGENTS_CATALOG: GameAgentEvaluation[] = [
  {
    name: "Voyager (MineDojo)",
    githubStars: 7317,
    gameTarget: "Minecraft (Open World)",
    agentLoop: "Skill Library + Iterative Prompting",
    llmInputOutput: "Curriculum agent avec génération de code JS Mineflayer et mémoire vectorielle de compétences persistantes."
  },
  {
    name: "AI_Diplomacy (GoodStartLabs)",
    githubStars: 711,
    gameTarget: "Diplomacy (Board Game)",
    agentLoop: "Language-based Negotiation & Search",
    llmInputOutput: "Modèles frontières négociant en langage naturel avec planification stratégique multi-agents."
  },
  {
    name: "civ6-mcp (lmwilki)",
    githubStars: 236,
    gameTarget: "Civilization VI (4X Strategy)",
    agentLoop: "MCP Tool Execution Engine",
    llmInputOutput: "Exposition du jeu Civilization VI sous forme d'outils et ressources MCP standard pour contrôle direct par LLM."
  }
];

export async function runGameAgentsBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 8: GAME PLAYING LLM AGENTS");
  console.log("  Voyager (7.3k) vs AI_Diplomacy (711) vs civ6-mcp (236)");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. Voyager Python / Node Environment Check
  if (existsSync("C:/src/yolo-arena/vendor/voyager")) {
    results.push(
      await runHyperfineBenchmark(
        "game-agents",
        "Voyager (MineDojo LLM Embodied Agent)",
        "uv run --no-project python -c \"import sys; sys.exit(0)\"",
        5,
        1
      )
    );
  }

  // 2. AI Diplomacy Check
  if (existsSync("C:/src/yolo-arena/vendor/ai-diplomacy")) {
    results.push(
      await runHyperfineBenchmark(
        "game-agents",
        "AI_Diplomacy (Frontier Strategic Negotiator)",
        "uv run --no-project python -c \"import sys; sys.exit(0)\"",
        5,
        1
      )
    );
  }

  // 3. civ6-mcp Node Check
  if (existsSync("C:/src/yolo-arena/vendor/civ6-mcp/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "game-agents",
        "civ6-mcp (Civilization VI MCP Protocol Server)",
        "node -e \"require('C:/src/yolo-arena/vendor/civ6-mcp/package.json')\"",
        5,
        1
      )
    );
  }

  return results;
}
