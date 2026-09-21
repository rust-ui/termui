# Changelog

Notable Term/UI changes are recorded here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Added a composable `Card` widget with header, title, description, action, content, footer, docs, and a Rust-rendered demo.
- Added ColorPicker, Dialog, RadioCard, and TextInput widgets with Rust-rendered demos and copyable source pages; ported Ferrit's licensed `tui_overlay` and animated error Toast, with native mouse dismissal.
- Split Ratatui frame rendering into `termui-renderer` and per-demo files plus ID dispatch into `termui-registry`; keep interactive and static demo sources together, and validate static previews from docs references during frame generation.
- Added terminal preview sections for every remaining widget in the all-widgets plan.
- Added Calendar and Chart sections to the all-widgets plan with terminal previews.
- Added a Checkbox section to the all-widgets plan with unchecked and checked states.
- Added separate Button default, variant, size, and interactive examples with a shared Preview/Code switcher; static examples render the actual widget API.
- Added `ButtonSize::{Sm, Default, Lg}` for terminal-cell padding, with a larger default button and Button previews.
- Added an interactive Ratatui button demo inside the terminal preview, running in Ratzilla/WebAssembly with checked-in web assets for production.
- Added a copyable Ratatui `Button` widget with shadcn-inspired color variants, focus and disabled states, docs, and a Rust-rendered ANSI preview.
- Added `AGENTS.md` and `__SKILLS_LEARNINGS/LEARNINGS.md` to preserve project rules and reusable implementation lessons.
- Added native Rust demo rendering that writes checked-in preview frames consumed by the site.
- Added a single dark/light theme toggle to the far right of the navigation.
- Added Docker/Compose production packaging and a GitHub Actions SSH deploy workflow for the shared Rustify server.

### Changed

- Adopted termcn’s geometric logo mark with a custom R, updating the favicon family and downloadable Term/UI logo.
- Rebuilt Dialog as composable trigger, content, header, title, description, footer, and close components; added a Ratzilla demo with a blue trigger that opens on click.
- Removed the KeyBar widget, page, component catalog entry, and preview.
- Renamed the Rust Radio Group page and preview to Radio Card; disable text selection in static terminal previews while leaving code samples selectable.
- Use the repository's `packageManager` pin as the sole pnpm version source in CI.
- Centered the interactive Button demo stack and grouped its controls more tightly.
- Marked Calendar, Chart, and Checkbox as ⚠️ Waiting in the all-widgets plan.
- Simplified the interactive Button preview to its Increment and Reset controls, a live counter, and click/keyboard instructions.
- Removed the Registry overview page and centered the initial launch on a small set of simple Ratatui widgets.
- Removed the duplicate Widgets link from the docs Sections navigation; widget links remain grouped below.
- Added a Usage section after Source on each widget page.
- Moved the button example into a dedicated Usage section after its source.
- Replaced Carbon Ads in the documentation with a Rustify CTA linking to `rustify.rs`.
- Added Ratatui widget pages under flat `/docs/widgets/*` routes with the termcn sidebar navigation and searchable widget source.
- Reduced public documentation to four core pages, redirected retired catalog routes to the registry, and retained the source-site changelog history.
- Set Term/UI as an independent product with `termui.rustify.app` as its canonical site, registry, SEO, and deployment origin.
- Replaced cloud-platform deployment with the shared-apps Docker, Docker Hub, SSH, Nginx, and Certbot deployment path.
- Upgraded the pinned pnpm toolchain to 12.5.1 and locked its Corepack binary metadata.
- Copied the termcn site UI and content while omitting its Sponsor page and navigation link.
- Replaced browser terminal runtimes and generated WebAssembly demos with static Rust-rendered frames.
- Aligned pnpm supply-chain protections with RUSTIFY; upgraded Next.js to 16.3.3 after the production audit found two critical advisories.
- Kept Term/UI's existing Ratatui component pages and source while moving them into the termcn documentation shell.

### Fixed

- Fixed the interactive Button demo’s WebAssembly crash by sizing its DOM terminal to the iframe viewport.
- Added the missing centralized registry docs route constant required by the API catalog.
- Removed stale OpenTUI JSX declarations that overrode React's HTML element props.
- Removed duplicate `installation` documentation slug.
- Bound legacy component preview MDX to the Rust frame renderer.
- Removed registry file tracing from fully static pages to avoid duplicating registry files into each docs route.

## Source history

The copied website changelog remains under [`content/docs/(root)/changelog/`](content/docs/(root)/changelog/).
