# Learnings

One line per reusable project lesson. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

## Confirmed

- 2026-09-21 [termui/deploy] bind app containers to localhost-only ports and route public traffic through host Nginx.
- 2026-09-21 [termui/deploy] build Next.js standalone output in Docker and run it as a non-root user on shared-apps.

## Inbox

- 2026-09-21 [termui/brand] present Term/UI as part of the Rust/UI ecosystem; use `rust-ui.com` as canonical domain and `@termui` as registry namespace.
- 2026-09-21 [termui/toolchain] keep `packageManager` aligned with the installed pnpm major; Corepack follows the repo pin even when a newer global pnpm exists.
- 2026-09-21 [termui/docs] check Fumadocs route groups when moving docs; `(root)` does not create a distinct URL segment, so same-slug pages collide.
