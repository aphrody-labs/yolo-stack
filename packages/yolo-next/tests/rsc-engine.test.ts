// SPDX-License-Identifier: Apache-2.0
import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { FULLSTACK_CANDIDATES, adaptFullstackModule, createInlineEngine, resolveRscEngine } from "../src/rsc-engine.ts";
import { removeDir, scratchDir } from "./helpers.ts";

let root: string;

beforeAll(() => {
  root = scratchDir("engine");
  mkdirSync(join(root, "app"), { recursive: true });
  writeFileSync(join(root, "app", "layout.tsx"), 'export const metadata = { title: "Engine" };\nexport default function L({ children }: { children: React.ReactNode }) { return <html lang="en"><body>{children}</body></html>; }');
  writeFileSync(join(root, "app", "page.tsx"), "export default function P() { return <main><h1>inline page</h1></main>; }");
});

afterAll(() => {
  if (root) removeDir(root);
});

const text = async (stream: ReadableStream<Uint8Array>): Promise<string> => new Response(stream).text();

describe("resolveRscEngine", () => {
  test("falls back to inline when no engine is available", async () => {
    const logs: string[] = [];
    const engine = await resolveRscEngine({ root, candidates: [join(root, "nope"), "/definitely/not/here"], packageResolve: false, bundles: false, log: (m) => logs.push(m) });
    expect(engine.name).toBe("inline");
    expect(logs).toContain("rsc engine: inline");
    const html = await text(await engine.renderHtml({ request: new Request("http://localhost/") }));
    expect(html).toContain("<title>Engine</title>");
    expect(html).toContain("<h1>inline page</h1>");
    const flight = await text(await engine.renderFlight({ request: new Request("http://localhost/") }));
    expect(flight.startsWith('0:["$","div",null,')).toBe(true);
    expect(flight).toContain("inline page");
    const missing = await text(await engine.renderHtml({ request: new Request("http://localhost/nope") }));
    expect(missing).toContain("404");
  }, 30_000);

  test("prefer inline short-circuits", async () => {
    const engine = await resolveRscEngine({ root, prefer: "inline" });
    expect(engine.name).toBe("inline");
    expect(createInlineEngine({ root }).name).toBe("inline");
    expect(FULLSTACK_CANDIDATES(root)[0]?.replace(/\\/g, "/")).toBe(`${root.replace(/\\/g, "/")}/node_modules/@yolo-rs/fullstack`);
  });

  test("picks a fullstack candidate directory exposing loadApp", async () => {
    const candidate = join(root, "fake-fullstack");
    mkdirSync(join(candidate, "src", "app"), { recursive: true });
    writeFileSync(join(candidate, "src", "app", "index.ts"), 'export async function loadApp(outDir: string) { return { fetch: async (request: Request) => new Response(request.headers.get("rsc") ? "0:flight\\n" : "<html>fake " + outDir.length + "</html>") }; }\n');
    const engine = await resolveRscEngine({ root, candidates: [candidate], packageResolve: false, bundles: false, fullstackOutDir: join(root, "fs-out") });
    expect(engine.name).toBe("fullstack");
    expect(engine.details.source?.replace(/\\/g, "/")).toBe(join(candidate, "src", "app", "index.ts").replace(/\\/g, "/"));
    expect(await text(await engine.renderFlight({ request: new Request("http://localhost/") }))).toBe("0:flight\n");
    expect(await text(await engine.renderHtml({ request: new Request("http://localhost/") }))).toContain("<html>fake");
  });
});

describe("adaptFullstackModule", () => {
  test("wraps a fake engine module", async () => {
    const seen: Array<{ url: string; rsc: string | null; action: string | null }> = [];
    let built = 0;
    const fake = {
      buildApp: async () => {
        built += 1;
      },
      loadApp: async () => ({
        fetch: async (request: Request) => {
          seen.push({ url: request.url, rsc: request.headers.get("rsc"), action: request.headers.get("yolo-action") });
          return new Response(request.headers.get("rsc") ? "0:payload\n" : "<!doctype html><p>page</p>", { headers: { "content-type": request.headers.get("rsc") ? "text/x-component" : "text/html" } });
        },
      }),
    };
    const engine = adaptFullstackModule(fake, { root, outDir: join(root, "fs-out") });
    expect(engine?.name).toBe("fullstack");
    expect(await text(await engine!.renderFlight({ request: new Request("http://localhost/a") }))).toBe("0:payload\n");
    expect(await text(await engine!.renderHtml({ request: new Request("http://localhost/b") }))).toBe("<!doctype html><p>page</p>");
    const action = await engine!.handleAction!(new Request("http://localhost/c", { method: "POST", headers: { "next-action": "00abc" } }), {});
    expect(action.status).toBe(200);
    const whole = await engine!.fetch!(new Request("http://localhost/d"));
    expect(whole.headers.get("content-type")).toBe("text/html");
    expect(built).toBe(1);
    expect(seen.map((s) => s.rsc)).toEqual(["1", null, null, null]);
    expect(seen[2]?.action).toBe("00abc");
    expect(adaptFullstackModule({}, { root })).toBeUndefined();
    expect(adaptFullstackModule(null, { root })).toBeUndefined();
  });

  test("render failures fall back to the inline engine", async () => {
    const broken = { loadApp: async () => { throw new Error("no build"); } };
    const engine = adaptFullstackModule(broken, { root, outDir: join(root, "fs-broken") });
    const html = await text(await engine!.renderHtml({ request: new Request("http://localhost/") }));
    expect(html).toContain("<h1>inline page</h1>");
  }, 30_000);
});
