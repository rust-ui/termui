# Term/UI agent guide

- Keep project documentation and generated user-facing copy in English.
- Treat Term/UI as an independent product. Use `termui.rustify.app` as canonical domain and `@termui` as registry namespace.
- Add a concise entry under `## [Unreleased]` in `CHANGELOG.md` for every visible or architectural change.
- Add reusable project lessons to `__SKILLS_LEARNINGS/LEARNINGS.md`; keep one dated line per lesson.
- Preserve the copied termcn UI and behavior. Sponsor page and Sponsor link stay omitted.
- Default demos to Rust-generated static text frames. Use Ratzilla/WebAssembly when clicking or keyboard input meaningfully changes widget state; keep visual-only variant and size demos static.
- Keep Rust demo frames and built interactive demo assets checked in. The production Next.js image uses `pnpm build:production` and does not require a Rust toolchain.
- Keep pnpm supply-chain policies aligned with RUSTIFY. Fix vulnerable dependencies without weakening release-age or trust policies.
- Preserve `content/docs/(root)/changelog/**`; this is the source site's visible changelog.
- Deploy through the GitHub Actions Docker/SSH workflow to shared-apps. Keep the Dockerfile, Compose file, Nginx config, and workflow aligned.

## Before writing code

Use the smallest existing mechanism that solves the task. Check existing components, scripts, and dependencies before adding abstractions or packages. Explain new dependencies or one-caller abstractions in the changelog.

## Learnings

When a recurring project preference or implementation lesson emerges, add one dated line under `Inbox` in `__SKILLS_LEARNINGS/LEARNINGS.md`. Skip one-off facts and rules already enforced by config or CI.
