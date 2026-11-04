// Framework-free Bun.serve baseline. Renders the shared route set as static HTML strings so the
// harness has a ceiling to compare every stack against. Not an application.
import { ROUTES } from "@yolo-rs/stack-routes";

const port = Number(process.env.PORT ?? 4100);

function page(id: string, rows: number): string {
  let body = `<h1>${id}</h1>`;
  if (rows > 0) {
    body += "<ul>";
    for (let i = 0; i < rows; i++)
      body += `<li><a href="/row/${i}">row ${i}</a></li>`;
    body += "</ul>";
  }
  return `<!doctype html><html><head><meta charset="utf-8"><title>${id}</title></head><body>${body}</body></html>`;
}

const routes: Record<string, () => Response> = {};
for (const r of ROUTES) {
  if (r.id === "api-json") {
    routes[r.path] = () => Response.json({ ok: true, ts: Date.now() });
  } else {
    const html = page(r.id, r.rows);
    routes[r.path] = () =>
      new Response(html, {
        headers: { "content-type": "text/html; charset=utf-8" },
      });
  }
}

const server = Bun.serve({
  port,
  routes,
  fetch: () => new Response("not found", { status: 404 }),
});
console.log(`baseline listening on ${server.url}`);
