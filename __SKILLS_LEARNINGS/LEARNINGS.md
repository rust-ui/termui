# Learnings

One line per reusable project lesson. Format: `YYYY-MM-DD [domain] avoid X, do Y, because Z`.

## Confirmed

- 2026-09-21 [termui/deploy] align the pnpm pin with RUSTIFY at 11.24.0 when the installed Corepack cannot launch pnpm 12.
- 2026-09-21 [termui/deploy] bind app containers to localhost-only ports and route public traffic through host Nginx.
- 2026-09-21 [termui/deploy] build Next.js standalone output in Docker and run it as a non-root user on shared-apps.

## Inbox

- 2026-09-21 [termui/docs] check Fumadocs route groups when moving docs; `(root)` does not create a distinct URL segment, so same-slug pages collide.
