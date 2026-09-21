# Plan: TERMUI with termcn’s verbatim UI

Source of truth: `__TMP/termcn/`, especially its Next.js app at `apps/web/`.

## Goal and invariants

- Reproduce all of termcn’s UI and behavior in TERMUI: pages, navigation, responsive layouts, themes, transitions, docs, components, charts, registry, search, sharing, shortcuts, and visual content.
- Keep termcn’s UI code, markup, class names, copy, and behavior verbatim wherever possible. Record every necessary deviation. Do not redesign or simplify.
- Do not create or migrate the Sponsor page. Remove its navigation, links, metadata, sitemap entries, and other references that lead to a missing page.
- Replace only the component-demo rendering with Rust-produced output. Do not use Ratzilla or Ratatui. Keep React/Next responsible for the website UI and its chrome.
- Keep termcn’s Ink/OpenTUI variants and routes so the site remains complete. Where those pages show executable demos, label and connect previews honestly to Rust examples; do not claim Rust is Ink/OpenTUI.
- Add a small, isolated Rust-to-browser renderer. Rust produces a versioned frame/cell model (text, styles, dimensions, state); the existing termcn preview components display it. No arbitrary Rust execution on the server.

## Target architecture

1. Bring `__TMP/termcn/apps/web` into TERMUI’s web-app root. Preserve its `app/`, `components/`, `content/`, `lib/`, `registry/`, `styles/`, and `public/` structure.
2. Merge its web dependencies and scripts into TERMUI’s root package setup. Avoid retaining monorepo tooling that the single-app repo does not need.
3. Keep all termcn UI components as the website layer. Replace only the demo engine behind its existing preview surfaces with the Rust renderer.
4. Generate a static frame index with a native Rust renderer during `demos:build`; do not add Ratzilla or Ratatui. The existing preview UI displays the Rust-generated frames and keeps its theme/size controls.
5. Preserve every non-Sponsor termcn route and feature. Update TERMUI identity, URLs, and repository links only where required.

## File-by-file migration map

Paths in the Source column are relative to `__TMP/termcn/`. Target paths are relative to TERMUI’s root. “Copy tree” means copy every relevant file and nested directory, preserving names and structure, then make only documented technical/brand adaptations.

### Repository and build files

| Source | Target action |
|---|---|
| `package.json` | Merge web scripts and dependencies into TERMUI’s root package. Keep dev/build/start/typecheck and example/registry build scripts. Exclude Sponsor-only scripts. |
| `pnpm-workspace.yaml`, `turbo.json` | Keep workspace/Turbo only if the resulting layout needs it; do not add a needless monorepo wrapper. |
| Root `tsconfig.json`, formatter/linter/hook configs | Reuse only required settings and fix paths for the single TERMUI app. |
| `apps/web/next.config.mjs`, `tsconfig.json`, `postcss.config.mjs`, `components.json`, `source.config.ts` | Copy to the matching TERMUI web root and adjust only root-relative paths. |
| `pnpm-lock.yaml` | Regenerate once after dependency merge; do not copy the monorepo lockfile as-is. |
| `.gitignore`, `SECURITY.md`, README and other root docs/config | Merge required entries and rewrite commands/identity for TERMUI. |

### App routes and page layouts

| Source | Target action |
|---|---|
| `apps/web/app/layout.tsx` | Copy global metadata, fonts, providers, theme, and document shell. |
| `apps/web/app/(app)/layout.tsx` | Copy shared site shell, header, footer, nav, and transitions. |
| `apps/web/app/(app)/(root)/page.tsx` | Copy the full home page and all sections/CTAs; change only demo engine bindings. |
| `apps/web/app/(app)/docs/layout.tsx` and `docs/[[...slug]]/page.tsx` | Copy docs shell, sidebar, TOC, MDX route, metadata, and page tree. |
| `apps/web/app/(app)/docs/changelog/page.tsx` | Copy changelog page and content. |
| `apps/web/app/(app)/launch-week/page.tsx` and `[week]/page.tsx` | Copy routes and behavior. |
| `apps/web/app/not-found.tsx` | Copy 404 states and links. |
| `apps/web/app/api/status/route.ts`, `openapi.json/route.ts`, `.well-known/**` | Copy utility/discovery routes; adapt TERMUI identity and any inaccurate implementation details. |
| `apps/web/app/llms*.txt/**`, `apps/web/app/llms.md/**` | Copy discovery routes and regenerate content from TERMUI routes/docs. |
| `apps/web/app/robots.txt/route.ts`, `sitemap.ts`, `manifest.ts`, `rss.xml/route.ts` | Copy and adapt domain, route list, and entries; exclude Sponsor. |
| `apps/web/app/(app)/sponsor/page.tsx` | Do not copy. Remove its route and every generated reference. |
| All remaining files in `apps/web/app/**` | Inventory and copy every non-Sponsor route. Document any intentionally omitted route. |

### Shared UI components

| Source | Target action |
|---|---|
| `apps/web/components/**` | Copy the full component tree; preserve markup, props, classes, animation, responsive behavior, and states. Apply exclusions below only. |
| `apps/web/components/ui/**` | Copy every used shadcn/Radix primitive unchanged; align dependency versions/imports as required. |
| `apps/web/components/animated-icons/**`, `daikanoid/**` | Copy animations/game and keep their visual behavior. |
| `ink-preview.tsx`, `opentui-preview.tsx`, `terminal-preview.tsx`, `component-preview.tsx`, `example-preview.tsx`, `mac-window.tsx`, `terminal-theme.tsx` | Copy first. Change only the demo engine seam to consume Rust output; retain preview chrome, sizing, controls, theme UI, and surrounding markup. |
| `command-menu.tsx`, `code-tabs.tsx`, `code-block-command.tsx`, `code-collapsible-wrapper.tsx`, `copy-button.tsx` | Copy behavior and appearance; adapt command text only when required for Rust/TERMUI installation. |
| Navigation/header/footer/docs components | Copy verbatim. Remove Sponsor entries only; retain all other termcn categories and navigation. |
| Theme/settings components | Copy theme provider, switchers, settings, and theme previews without visual changes. |
| Home/hero/callout/announcement/logo/icon components | Copy all; change only TERMUI brand, domain, and repository destinations. |
| Registry install/command components | Keep the UI; make displayed commands accurate for TERMUI copy-paste usage. |
| Ads, analytics, Sponsor links, private audio/haptics integrations | Retain only features that work with available TERMUI services. Omit Sponsor and inaccessible private integrations; record each resulting deviation. |

### Styles, fonts, constants, and libraries

| Source | Target action |
|---|---|
| `apps/web/styles/globals.css` | Copy full tokens, light/dark themes, animations, breakpoints, code/terminal styles. No restyling. |
| `apps/web/lib/fonts.ts`, `utils.ts` | Copy fonts and shared UI utilities. |
| `apps/web/lib/highlight-code.ts`, `format-code.ts` | Copy syntax highlighting/formatting; ensure Rust syntax is enabled without changing visual themes. |
| `apps/web/lib/docs.ts`, `source.ts`, `page-tree.ts`, `read-file.ts` | Copy docs/source pipeline and adjust only project paths/aliases. |
| `apps/web/lib/terminal-themes.ts` and `terminal-themes/**` | Copy all themes and feed the same theme data into Rust previews. |
| `apps/web/lib/api.ts`, `github.ts`, `url.ts`, `events.ts`, `changelog.ts`, `launch-week.ts`, `registry.ts` | Copy helpers needed by retained features; adapt origin/repo/branch and remove Sponsor data. |
| `apps/web/lib/ink-web-adapter.ts`, `lib/opentui-bridge/**` | Do not use these as the runtime demo renderer. Retain only if docs/tooling still imports them. |
| `apps/web/constants/{site,links,nav,routes}.ts` | Copy shapes and route data; set TERMUI identity/domain/GitHub and remove Sponsor only. |
| `apps/web/seo/{metadata.ts,json-ld.tsx}` | Copy SEO implementation; adapt TERMUI identity and routes, with no Sponsor URL. |
| `apps/web/registry/config.ts`, `registry/bases.ts` | Copy registry configuration and retain both visible Ink/OpenTUI bases. Point executable preview entries to Rust examples without mislabeling them. |

### Documentation and registry content

| Source | Target action |
|---|---|
| `apps/web/content/docs/**` | Copy every MDX, metadata JSON, and related asset, including changelog, charts, themes, components, both Ink/OpenTUI trees, MCP, registry, and theming. Preserve titles, order, text, and navigation. |
| `apps/web/content/docs/(root)/**` | Keep index, installation, MCP, registry, theming, and changelog. Correct only claims/commands that would falsely describe TERMUI’s renderer. |
| `apps/web/content/docs/components/{ink,opentui}/**` | Copy all component categories, index pages, and metadata. Replace executable demo/source bindings only where needed; preserve docs chrome and prose. |
| `apps/web/content/docs/charts/{ink,opentui}/**`, `themes/{ink,opentui}/**` | Copy all pages/assets/metadata and preserve theme controls/routes; connect previews to Rust examples where applicable. |
| `apps/web/content/docs/meta.json`, `source.config.ts`, `mdx-components.tsx` | Copy Fumadocs/MDX config and component mappings, updating only project paths. |
| `apps/web/registry/**` | Copy the complete registry tree (367 files at inventory time), including both bases, components, charts, utilities, and themes. Preserve names and metadata. |
| `apps/web/public/r/**` | Copy all registry JSON assets (345 files at inventory time); preserve public URLs and update only content/URLs that must identify TERMUI accurately. |
| `apps/web/examples/**`, `scripts/build-examples.mts`, `scripts/build-registry.mts` | Copy examples/generation. Add Rust source/build output as canonical demo input and keep generated registry paths stable. |
| `apps/web/public/**` | Copy static assets; replace termcn brand assets with TERMUI assets where branding appears, keeping dimensions and usage. |

### New Rust renderer files

Add an isolated crate and web adapter, for example:

| New file | Responsibility |
|---|---|
| `crates/termui-renderer/Cargo.toml` | Rust renderer crate for native/WASM targets; no Ratatui/Ratzilla dependencies. |
| `crates/termui-renderer/src/lib.rs` | Rust frame-generation API: glyphs, display widths, colors/attributes, dimensions, and named examples. |
| `crates/termui-renderer/src/examples/**` | Rust examples that provide the demo content and state transitions shown by existing previews. |
| `crates/termui-renderer/src/main.rs` | Native CLI serializes all discovered example frames as JSON. |
| `apps/web/lib/rust-renderer/protocol.ts` | TypeScript key/types for Rust-generated frame data. |
| `apps/web/lib/rust-renderer/previews.generated.json` | Checked-in build output containing frames generated from the Rust renderer. |
| `apps/web/components/rust-preview.tsx` | Thin adapter into existing termcn preview slots/classes; add no new visual system. |
| `apps/web/scripts/build-rust-demos.mts` | Discover example names, run the Rust CLI once, and write deterministic JSON before dev/build. |
| `crates/termui-renderer/README.md` | Document protocol/build limits accurately; do not describe it as Ratatui/Ratzilla. |

Rust generates frames at build time; React renders their text in the existing termcn preview shell. This is static preview output, not live Rust execution in the browser. Do not use Ratzilla or Ratatui. Keep this boundary explicit in docs.

### Existing TERMUI files to replace or retire

- Replace existing site files with their termcn counterparts only after tracing imports: `app/page.tsx`, current docs routes/content, `components/terminal-frame.tsx`, `components/component-source.tsx`, current nav/header/footer/logo/UI components, `lib/**`, `constants/**`, `app/globals.css`, `mdx-components.tsx`, `source.config.ts`, Next/package config.
- Keep `components/terminal-frame.tsx` or `component-source.tsx` only if no exact termcn equivalent exists and they are required by the copied app; prefer termcn components.
- Retire `crates/termui-demo*`, `crates/termui-widgets`, and `public/demos/**` only after Rust renderer examples replace every reference and the copied site builds.
- Remove Ratzilla/Ratatui dependencies and claims after runtime migration. Keep Ink/OpenTUI docs/routes/assets; they are part of the verbatim UI/content scope.
- Do not overwrite or discard unrelated user changes. Inventory current dirty files before each replacement and preserve changes not created by this migration.

## Execution loop

1. Inventory source routes/files/dependencies and current TERMUI changes.
2. Copy the full non-Sponsor app/content/assets and merge package/config files.
3. Build and resolve imports/types while preserving source UI.
4. Compare route lists, then compare screenshots at matching viewports and themes; fix every unexplained UI/behavior difference.
5. Implement the native Rust frame generator and connect its output through existing preview surfaces.
6. Migrate all demo surfaces and verify frame sizing, Unicode widths, colors, and theme switching.
7. Apply only TERMUI branding/domain/repository updates; remove Sponsor references.
8. Retire old Ratzilla demos/dependencies only after no references remain.
9. Repeat typecheck/build, route scan, Sponsor/Ratzilla scan, and desktop/mobile visual/interaction review until clean.

## Acceptance criteria

- Every non-Sponsor termcn route, page, interaction, layout, theme, responsive state, and content category exists in TERMUI and matches termcn.
- No intentional visual redesign; every necessary deviation is recorded.
- Sponsor page and all links/SEO/generated references to it are absent.
- Demos display Rust-produced frames through the existing preview UI, with no Ratzilla/Ratatui runtime and no misleading claim that Rust is Ink/OpenTUI or executes live in the browser.
- Menus, search, copy/share actions, transitions, docs, themes, and demo controls work.
- `pnpm typecheck` and `pnpm build` pass; route/content inventory and visual review show no unexplained gaps.
