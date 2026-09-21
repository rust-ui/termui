# Divergences from termcn

Goal is verbatim-as-possible parity with termcn (shadcn-labs), rendering
Ratatui/WASM instead of Ink/OpenTUI/React. This file tracks every place
termui deliberately differs, and why. Update it as more gets built.

## Dropped entirely (not applicable / private / brand)

- **`@web-kits/audio` (`useFeedback`, `SoundProvider`, `/audio/core` sound
  defs)** — private package, click/toggle/drawer sound effects. Stripped
  from `button.tsx`, `sheet.tsx`, `sidebar.tsx`, `mobile-nav.tsx`,
  `mode-switcher.tsx`, `site-footer.tsx`, `github-stars.tsx`.
- **Haptics** (`useHapticsToggle`, `HapticsSwitcher`) — same reason, no
  equivalent needed.
- **`LabsNav`, `LABS_*` constants** — cross-promo links to sibling
  shadcn-labs commercial products (shadercn, framecn, ogimagecn, etc.).
  No sibling product suite exists for termui.
- **`SponsorLink`, GitHub Sponsors links, `Analytics` component** — not
  set up for this project yet.
- **Literal `LogoMark`/`LogoType` SVGs** — termcn's actual brand/trademark
  glyphs. Replaced with an original pixel-block "termui" mark
  (`components/logo.tsx`) that keeps the visual language (blocky pixel
  construction) but not the shape.
- **Ink/OpenTUI dual-renderer split** (`registry/bases/{ink,opentui}`,
  `content/docs/{components,charts,templates,themes}/{ink,opentui}`,
  `ComponentsSidebarPanel`/`TemplatesSidebarPanel`/`ChartsSidebarPanel`/
  `ThemesSidebarPanel` switching logic) — termui has one renderer
  (Ratatui/WASM) and one component category. `docs-sidebar.tsx` and
  `mobile-nav.tsx` are flat single-group lists instead.
- **Charts, Templates, Themes sections** — not part of termui's scope.
- **`Inter` font** — termcn combines Geist Sans, Geist Mono, and Inter;
  termui only needed Geist Sans + Geist Mono, so `Inter` was dropped from
  `lib/fonts.ts`.

## Simplified (same intent, less machinery)

- **`constants/routes.ts` / `constants/nav.ts`** — termcn's versions carry
  ~20 route constants (charts, registry, MCP, launch-week, agent-skills,
  sponsor, RSS, llms.txt variants...). termui's carries only
  `HOME`/`DOCS`/`DOCS_INSTALLATION`/`DOCS_COMPONENTS`.
- **`components/mobile-nav.tsx`** — termcn's is ~220 lines with
  panel-switching logic per top-level section. termui's keeps the
  animated hamburger icon and `Sheet` shell verbatim in spirit, but
  renders one flat link list (no panel switching needed since there's
  only one content category).
- **`components/github-stars.tsx`** — dropped the `UTM_PARAMS`/
  `addQueryParams` query-string tracking (termcn's is tied to their
  analytics setup); links straight to the GitHub repo.
- **`lib/github.ts`** — dropped filtering stargazers by `GITHUB.user`
  (termcn's constant naming was inconsistent between org/user; termui's
  `GITHUB` const only has `org`/`repo`/`branch`).
- **GitHub icon** — termcn imports a `GithubIcon` from their own
  hand-drawn `icons.tsx` (28.8K). The installed `lucide-react` version
  here (1.7.0) also ships no brand/logo icons. Used a small inline SVG
  path instead of pulling in a whole icon-set file for one glyph.

## Deferred (real gap, not yet built)

- **View Transitions page-navigation animations** — termcn's
  `DirectionalTransition` uses React's experimental `<ViewTransition>`
  (fade/slide/scale/morph on nav). That API isn't stable in the installed
  React 19.2.4, so `components/directional-transition.tsx` is currently a
  no-op passthrough. Revisit once React's ViewTransition ships stable, or
  reimplement by hand with the browser View Transitions API + CSS.
- **`CommandMenu` (cmd+k search)** — not built yet.
- **Component registry / CLI installer** (`npx shadcn add <url>`
  equivalent) — no direct cargo/Rust analogue exists yet. Components are
  meant to be copy-pasted from the docs source blocks in the meantime.
- **Full MDX content pipeline** (`source.config.ts`, `fumadocs-mdx`,
  `content/docs/**`, catch-all `app/docs/[[...slug]]/page.tsx`,
  `mdx-components.tsx`) — dependencies installed, not yet wired up. Docs
  pages are still the old hardcoded `app/docs/{panel,key-bar,select-list}
  /page.tsx` files.
- **`Drawer` UI primitive** — termcn's `site-settings.tsx` uses a mobile
  `Drawer` (vaul) + desktop `Popover` responsive pair. Not added yet
  (would pull in the `vaul` dependency); current settings surface is just
  the header's `ModeSwitcher` button group, no drawer/popover wrapper.
- **SEO/discoverability infra** — `llms.txt`, `llms-full.txt`,
  `robots.ts`, `sitemap.ts`, OG images, JSON-LD. Not started.
- **Installation docs content** — still shows the placeholder
  `cargo add ratatui` line; needs real copy-paste-into-your-app
  instructions once the MDX pipeline lands.

## Renamed / re-scoped, same shape

- **`GITHUB` org/repo** — pointed at `rust-ui/termui` instead of
  `shadcn-labs/termcn`.
- Home page showcase grid will use termui's 3 existing components
  (Panel, Key Bar, Select List) in place of termcn's registry items
  (table, bar-chart, spinner, alert, tool-call, badge) — not yet wired,
  home page rebuild is still pending.
