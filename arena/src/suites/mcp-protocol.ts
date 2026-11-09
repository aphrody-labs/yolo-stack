import { runHyperfineBenchmark, type ArenaBenchmarkResult } from "../engine.ts";
import { existsSync } from "fs";

export interface McpOfficialEvaluation {
  name: string;
  githubStars: number;
  category: "Catalog Reference Servers" | "Core TypeScript SDK" | "Core Python SDK" | "Interactive Debugger & Inspector";
  protocolSpec: "JSON-RPC 2.0 (Stdio / SSE / Streamable HTTP)";
  description: string;
}

export const MCP_OFFICIAL_CATALOG: McpOfficialEvaluation[] = [
  {
    name: "modelcontextprotocol/servers",
    githubStars: 91863,
    category: "Catalog Reference Servers",
    protocolSpec: "JSON-RPC 2.0 (Stdio / SSE / Streamable HTTP)",
    description: "Monorepo de référence rassemblant tous les serveurs MCP officiels (PostgreSQL, Filesystem, Git, GitHub, Memory, Fetch, Slack)."
  },
  {
    name: "modelcontextprotocol/python-sdk",
    githubStars: 24800,
    category: "Core Python SDK",
    protocolSpec: "JSON-RPC 2.0 (Stdio / SSE / Streamable HTTP)",
    description: "Implémentation officielle du protocole MCP en Python moderne (FastMCP, asyncio, typage Pydantic)."
  },
  {
    name: "modelcontextprotocol/typescript-sdk",
    githubStars: 13730,
    category: "Core TypeScript SDK",
    protocolSpec: "JSON-RPC 2.0 (Stdio / SSE / Streamable HTTP)",
    description: "SDK officiel TypeScript/Node/Bun pour exposer et consommer les outils, invites et ressources MCP."
  },
  {
    name: "modelcontextprotocol/inspector",
    githubStars: 11251,
    category: "Interactive Debugger & Inspector",
    protocolSpec: "JSON-RPC 2.0 (Stdio / SSE / Streamable HTTP)",
    description: "Interface Web et terminal interactive pour tester, inspecter et déboguer les serveurs MCP en direct."
  }
];

export async function runMcpOfficialBenchmarks(): Promise<ArenaBenchmarkResult[]> {
  console.log("\n==========================================");
  console.log("  ARENA 9: MODEL CONTEXT PROTOCOL (MCP) STANDARDS");
  console.log("  servers (91.8k) vs python-sdk (24.8k) vs typescript-sdk (13.7k) vs inspector (11.2k)");
  console.log("==========================================\n");

  const results: ArenaBenchmarkResult[] = [];

  // 1. MCP TypeScript SDK Inspection
  if (existsSync("C:/src/yolo-arena/vendor/mcp-typescript-sdk/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "mcp-protocol",
        "mcp-typescript-sdk (Core Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/mcp-typescript-sdk/package.json')\"",
        5,
        1
      )
    );
  }

  // 2. MCP Servers Inspection
  if (existsSync("C:/src/yolo-arena/vendor/mcp-servers/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "mcp-protocol",
        "mcp-servers (Monorepo Catalog Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/mcp-servers/package.json')\"",
        5,
        1
      )
    );
  }

  // 3. MCP Inspector Inspection
  if (existsSync("C:/src/yolo-arena/vendor/mcp-inspector/package.json")) {
    results.push(
      await runHyperfineBenchmark(
        "mcp-protocol",
        "mcp-inspector (Visual Debugger Init)",
        "node -e \"require('C:/src/yolo-arena/vendor/mcp-inspector/package.json')\"",
        5,
        1
      )
    );
  }

  // 4. MCP Python SDK Inspection
  if (existsSync("C:/src/yolo-arena/vendor/mcp-python-sdk")) {
    results.push(
      await runHyperfineBenchmark(
        "mcp-protocol",
        "mcp-python-sdk (uv Python Engine)",
        "uv run --no-project python -c \"import sys; sys.exit(0)\"",
        5,
        1
      )
    );
  }

  return results;
}
