# termui

Copy-paste terminal UI components for Ratatui, rendered live in the browser
over WebAssembly.

Unlike a docs site that replays a recorded terminal session, the demos here
run the actual widget code: each component compiles to WASM via
[ratzilla](https://github.com/ratatui/ratzilla) and mounts into the page
through a real `ratatui::Terminal`.

## Layout

- `rust/crates/termui-widgets` — the components themselves. Struct + builder
  API + `render(frame, area)`, meant to be copied into your own app, not
  pulled in as a dependency.
- `rust/crates/termui-demo*` — one small binary per component, each compiled
  to WASM with `trunk` and embedded in the docs site as a live iframe.
- `web` — the Next.js/TypeScript docs site (Tailwind v4, shadcn conventions).

## Components

- Panel — rounded bordered panel shell
- Key Bar — bottom key-hint bar
- Select List — keyboard-navigable list

## Developing

```sh
# rebuild a demo's WASM bundle
cd rust/crates/termui-demo && trunk build --release --dist ../../../web/public/demos/panel

# run the docs site
cd web && pnpm install && pnpm dev
```
