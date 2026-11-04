# packages/

Bun/TypeScript side of the yolo framework. Bun 1.4.4 workspaces, `bun check` / `bun lint` /
`bun fmt`, typed TSX only (no raw HTML/CSS/JS files). No npm, npx, yarn, pnpm or Node.

| Package                                                 | Status           | Owner / merge path                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@yolo-rs/stack-routes` (`yolo-stack-routes`)           | compiles, tested | Shared showcase route table. Twin of `crates/yolo-stack-routes`.                                                                                                                                                                                                                                                                                                                                   |
| `@yolo-rs/stack-tanstack` (`yolo-stack-tanstack`)       | planned          | TanStack Router + Query + Start adapter on Bun's native fullstack server (`Bun.serve` routes + HTML imports, React 19). Replaces the Vite/Nitro `bun` preset with a `Bun.build` three-pass build and a `Bun.serve` request handler. Consumes the TanStack Query/Router adapters that `@yolo-rs/fullstack` already exposes (`C:/src/yolo-ui/packages/yolo-fullstack/src/tanstack`).                 |
| `@yolo-rs/stack-next-compat` (`yolo-stack-next-compat`) | planned          | Next.js App Router migration path. Thin consumer of `@yolo-rs/fullstack` (RSC via `react-server-dom-parcel`, server actions, streaming SSR, prerender, `next.config` loader, `next/font`, `next/image`). Linked as `file:../../../yolo-ui/packages/yolo-fullstack` until `@yolo-rs/fullstack` has a published `package.json`; then `workspace:*` through a Bun workspace link or a git dependency. |

## Merge path into the final framework

1. `@yolo-rs/stack-tanstack` is the app shell: file routes generated from `@yolo-rs/stack-routes`,
   `createServerFn` server functions, TanStack Query for data, Bun HMR in dev.
2. `@yolo-rs/stack-next-compat` lets an existing Next 16 `app/` tree run on the same `Bun.serve`
   process; routes migrate to TanStack one at a time while RSC pages keep working.
3. Both mount Rust islands through the `@yolo-rs/fullstack` Dioxus bridge
   (`C:/src/yolo-ui/packages/yolo-fullstack/src/dioxus`): `<Island name=... props=...>` renders the
   Rust-produced HTML during SSR and the `dx` web build hydrates it on the client.

Nothing is copied from `C:/src/yolo-ui` or `C:/src/bun-ui`; reference by path or dependency.
