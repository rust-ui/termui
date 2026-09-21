# Changelog

Notable TERMUI changes are recorded here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Added `AGENTS.md` and `__SKILLS_LEARNINGS/LEARNINGS.md` to preserve project rules and reusable implementation lessons.
- Added native Rust demo rendering that writes checked-in preview frames consumed by the site.
- Added a single dark/light theme toggle to the far right of the navigation.
- Added Docker/Compose production packaging and a GitHub Actions SSH deploy workflow for the shared Rustify server.

### Changed

- Replaced cloud-platform deployment with the shared-apps Docker, Docker Hub, SSH, Nginx, and Certbot deployment path.
- Upgraded the pinned pnpm toolchain to 12.5.1 and locked its Corepack binary metadata.
- Copied the termcn site UI and content while omitting its Sponsor page and navigation link.
- Replaced browser terminal runtimes and generated WebAssembly demos with static Rust-rendered frames.
- Aligned pnpm supply-chain protections with RUSTIFY; upgraded Next.js to 16.3.3 after the production audit found two critical advisories.
- Kept TERMUI's existing Ratatui component pages and source while moving them into the termcn documentation shell.

### Fixed

- Removed duplicate `installation` documentation slug.
- Bound legacy component preview MDX to the Rust frame renderer.
- Removed registry file tracing from fully static pages to avoid duplicating registry files into each docs route.

## Source history

The copied website changelog remains under [`content/docs/(root)/changelog/`](content/docs/(root)/changelog/).
