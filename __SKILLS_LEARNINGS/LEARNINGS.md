# Learnings

One line per reusable project lesson. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

## Confirmed

- 2026-09-21 [termui/deploy] bind app containers to localhost-only ports and route public traffic through host Nginx.
- 2026-09-21 [termui/deploy] build Next.js standalone output in Docker and run it as a non-root user on shared-apps.

## Inbox

- 2026-09-22 [termui/integrations] load live external counters through a client resource backed by a cached server endpoint; static layouts otherwise freeze the value at build time.
- 2026-09-21 [termui/content] mirror RUSTIFY's validated Fumadocs article model under `domains/` and `app/(seo)/`, but adapt author claims, CTAs, and UI to Term/UI's independent product.
- 2026-09-21 [termui/refactors] update test fixtures and generators alongside domain moves; lint/typecheck can pass while runtime checks still read old paths.
- 2026-09-21 [termui/overlays] hit-test outside clicks against `OverlayState::overlay_rect` after child controls; inside clicks and triggers must keep their own behavior.
- 2026-09-21 [termui/seo] use Search Console query/page data to choose Ratatui content work; improve component pages with distinct use cases and verified Rust examples instead of generic long-form word-count targets.
- 2026-09-21 [termui/widgets] compose stateful overlays from parent-owned state, trigger, content, and close controls; let the app route input so each part stays reusable.
- 2026-09-21 [termui/demos] route independent Ratzilla binaries through one shared static shell, but keep wasm-bindgen JS glue per binary because generated imports vary.
- 2026-09-21 [termui/demos] keep the Ratzilla terminal element flush with its viewport; padding shrinks the DOM grid while backend mouse coordinates still scale against the full viewport.
- 2026-09-21 [termui/brand] treat Term/UI as an independent Ratatui product; use `termui.rustify.app` as canonical domain and describe widgets as copyable Rust source modules.
- 2026-09-21 [termui/toolchain] keep `packageManager` aligned with the installed pnpm major; Corepack follows the repo pin even when a newer global pnpm exists.
- 2026-09-21 [termui/docs] check Fumadocs route groups when moving docs; `(root)` does not create a distinct URL segment, so same-slug pages collide.
- 2026-09-21 [termui/types] derive route and widget types from centralized catalogs; avoid duplicated string literals that drift as the catalog grows.
- 2026-09-21 [termui/demos] register Rust demos by exact ID and keep each demo renderer separate; substring-based renderer chains drift as examples grow.
- 2026-09-21 [termui/rust] derive enum display labels with Strum so variant labels stay aligned without manual match arms.
- 2026-09-21 [termui/docs] use `RustDemo` on widget pages because `TerminalFrame` has no Code tab; reject bare `TerminalFrame` during demo generation.
- 2026-09-21 [termui/tests] keep executable tests under `tests/` and split component checks by widget; browser tests catch interactions that preview asset checks cannot.
- 2026-09-21 [termui/charts] keep chart families in named modules and share only series data; this keeps each renderer composable and keeps APIs discoverable.
