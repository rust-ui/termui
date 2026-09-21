# Changelog

Notable Term/UI changes are recorded here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Added a composable right/left Drawer with profile-trigger demo, close button, Escape, and overlay-bounds outside-click dismissal; dismissible Toasts now close on outside click too.
- Added a Ratatui form composition guide built from copyable widgets and app-owned state.
- Added keyboard-driven Ratzilla demos for stateful controls, pickers, navigation, text views, tables, chart tooltips, and overlays; checked in their WebAssembly assets.
- Added four independent interactive Toast demos; each opens from its centered trigger and shows its notification bottom-right.
- Added `ButtonShape::{Rectangular, Rounded}`; buttons default to rectangular fills.
- Added `CardBorderType::{Rounded, Square}`; cards default to rounded corners.
- Add local per-component tests for Button and Text Input interactions, plus Card's Rust preview.
- Added 35 planned widgets with copyable Rust source, documentation pages, and checked-in terminal previews.
- Added modular area, bar, line, pie, radar, radial, and tooltip charts with Rust-rendered gallery previews, family routes, and a code sheet.
- Added an interactive Ratatui Text Input demo with click, typing, field switching, and Backspace behavior; checked in its WebAssembly assets.
- Added a composable `Card` widget with a prominent title, an RSX-style `termui!` macro, docs, and a Rust-rendered demo; removed the unused `CardAction` slot.
- Added ColorPicker, Dialog, RadioCard, and TextInput widgets with Rust-rendered demos and copyable source pages; ported Ferrit's licensed `tui_overlay` and animated Toast surfaces with native mouse dismissal.
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
- Added an interactive Resizable Layout demo with a draggable divider, Tab activation, arrow-key resizing, and a live active state.

### Changed

- Reorganized app source into feature-owned `domains/` and cross-feature `shared/` modules; kept Next.js `app/` and shadcn UI at the repository root.
- Refined the Drawer demo trigger, centered hint, and panel contrast.
- Refined Ratatui SEO metadata, page copy, and GitHub discovery links; removed mismatched FAQ markup and fabricated sitemap modification dates.
- Added strict Biome and Knip checks, Rust formatting/lint/test gates, and repository Git hooks; retained the shadcn CLI and component configuration.
- Removed the unused Ink/OpenTUI registries, MCP install flow, and TSX demo pipeline; preserved the Ratatui theme selector, Rust previews, and interactive assets.
- Feature the Rust/UI site and theme-aware logo in the Labs Latest card.
- Add an Ecosystem column with Rust/UI, Leptos UI, and Rustify links.
- Add a slash separator after the Labs menu trigger.
- Route interactive demos through one shared HTML shell; preserve per-demo Rust binaries and WebAssembly assets.
- Keep the visible changelog focused on Term/UI updates; remove inherited release history.
- Moved the `termui-widgets` crate into `crates/termui-registry/src/widgets` and updated its workspace, dependency, and source references.
- Extract Resizable Layout interaction state and input handling from its renderer; derive state labels with Strum.
- Keep chart legends within plot bounds in area and line chart previews.
- Replaced the old header and downloadable logo mark with the pixel-grid R.
- Renamed the labeled metric `Gauge` widget and docs page to `Progress`.
- Added `DialogTrigger::style` and used white with black text for the Dialog demo trigger.
- Replaced the active favicon with the pixel-grid R from V4 and regenerated ICO, PNG, Apple touch, and Android icon assets.
- Reject widget docs that use `TerminalFrame` without a Preview/Code switcher.
- Generate Ratzilla demo entry HTML from one shared template and MDX references.
- Show whether each widget demo is interactive or static beside its Preview/Code tabs.
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

### Fixed

- Let Labs NavigationMenu finish its 300ms close animation before hiding.
- Restore NavigationMenu open/close animations when `viewport=false`.
- Gate Tooltip enter animations on Radix's `delayed-open` and `instant-open` states; keep exit animation on `closed`.
- Use checked, overflow-safe sizing for scrollbars and Ratatui ratio constraints.
- Keep the Ratzilla terminal viewport aligned with pointer coordinates so clicks hit the rendered controls.
- Keep Toast's native `crossterm` mouse events out of WebAssembly builds.
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
