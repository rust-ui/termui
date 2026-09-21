# Term/UI Articles and SEO Plan

**Status:** Proposed implementation plan
**Prepared:** 2026-09-21
**Product:** Term/UI
**Canonical origin:** `https://termui.rustify.app`

## 1. Objective

Build a complete, maintainable English article section for developers searching for Ratatui, Rust terminal UI, Rust TUI widgets, and practical terminal application guidance. Reuse the article system and editorial workflow from `RUSTIFY-APP/app_v2_website`, adapted to Term/UI's existing Next.js, Fumadocs, domain layout, shadcn UI, Ratatui docs, and independent product identity.

The deliverable is a useful article library, not a set of keyword landing pages. Each article must solve a distinct task, show working Rust examples, and connect readers to the matching Term/UI documentation and copyable widget source.

This plan defines the system and an initial 14-article backlog. It does not publish drafts or claim traffic/search volume. Validate query demand against Google Search Console and current search results before drafting and revisit the briefs as evidence arrives.

## 2. Guardrails

- Write the site, article, metadata, and UI copy in English.
- Use `termui.rustify.app` for all canonical and absolute URLs. Keep Term/UI an independent product; do not inherit Rustify product CTAs or branding.
- Preserve Term/UI's existing shadcn UI, site shell, theme behavior, and copied termcn visual conventions. Reuse RUSTIFY's information architecture and behavior; adapt the presentation to Term/UI.
- Describe Term/UI accurately: copyable, customizable Ratatui widget source and Rust-rendered previews. Do not imply it is a published Cargo crate.
- Do not introduce a dependency until the existing Fumadocs, Zod, Next.js, and shared UI mechanisms have been checked.
- Do not publish articles that merely rephrase docs, repeat one query target, invent benchmarks, or claim hands-on experience without evidence.
- Keep `content/docs/(root)/changelog/**` intact. Keep Sponsor page and Sponsor link omitted.
- Add a concise `CHANGELOG.md` entry for the visible article section and any implementation architecture change.

## 3. Reference implementation reviewed

Reference repository: `/Users/user/dev/1-RUST/RUSTIFY-APP/app_v2_website`.

Relevant implementation:

- `source.config.ts`: Zod schemas and Fumadocs collections, with article MDX in `public/articles/`.
- `src/domain/articles/{categories,content,source,types}.ts`: closed category taxonomy, generated Fumadocs source, sorted/cached article loading, summary/detail models.
- `app/(seo)/articles/page.tsx`: article hub with featured and spotlight content, category filtering, category browsing, collection metadata, and generated OG image.
- `app/(seo)/articles/[slug]/page.tsx`: statically generated article routes, metadata, author byline, breadcrumbs, TOC/share rail, MDX body, article JSON-LD, and 404 behavior.
- `app/(seo)/articles/category/[category]/page.tsx`: category landing pages with article cards and breadcrumbs.
- `app/(seo)/articles/**/opengraph-image.tsx`: route-specific social images.
- `app/(seo)/authors/{page.tsx,[slug]/page.tsx}`: author index, profile pages, written-article cards, and Person JSON-LD.
- `src/lib/authors.ts`, `src/lib/seo.ts`, and `app/sitemap.ts`: author identity, metadata/schema helpers, and content sitemap entries.
- `public/articles/*.mdx` and `public/articles/author-max-wells.webp`: article source and Max Wells portrait.

Term/UI already has the needed foundations: Fumadocs MDX and Zod in `source.config.ts`, generated `.source/server`, `domains/docs`, `app/(seo)/_lib/metadata.ts`, `app/(seo)/_lib/json-ld.tsx`, an App Router sitemap, a shared site header/footer, shadcn components, and `shared/lib/events.ts`.

## 4. Target architecture

Keep content and product logic in their own domains. Keep public routes and SEO metadata routes under the existing root `app/` convention.

```text
source.config.ts                         # Add validated articles collection
content/articles/*.mdx                   # Article source, beside content/docs
public/articles/author-max-wells.webp    # Reused approved author avatar
domains/articles/
  categories.ts                          # Small, closed article taxonomy
  content.ts                             # Cached summaries, detail lookup, ordering
  source.ts                              # Fumadocs loader at /articles
  types.ts                               # Summary/detail/article heading types
  authors.ts                             # Author profiles and lookup by name/slug
app/(seo)/
  layout.tsx                             # Existing Term/UI site shell for SEO pages
  articles/layout.tsx                    # Article-area page structure if needed
  articles/page.tsx                      # Discoverable article hub
  articles/[slug]/page.tsx               # Static article detail route
  articles/[slug]/opengraph-image.tsx    # Per-article fallback social image
  articles/category/[category]/page.tsx  # Populated category landing pages
  articles/opengraph-image.tsx           # Hub social image
  authors/page.tsx                       # Author index
  authors/[slug]/page.tsx                # Author bio and article archive
app/(seo)/sitemap.ts                     # Include canonical article/category/author URLs
shared/config/routes.ts                 # Central /articles and /authors paths
```

`app/(seo)/layout.tsx` must render the existing Term/UI shell once: site header, main content, footer, and any global tools required on public pages. Check the `(app)` layout before implementation; share its shell only if that removes real duplicated behavior. Do not accidentally omit the global navigation/footer or render a second header.

### RUSTIFY-to-Term/UI adaptation

| RUSTIFY pattern | Term/UI decision |
| --- | --- |
| Fumadocs MDX files under RUSTIFY's `public/articles` | Use `content/articles/` beside Term/UI's existing `content/docs/`; keep article content separate from public image assets. |
| Strict Zod article frontmatter and closed categories | Add an `articles` collection in `source.config.ts`; validate every required field and category at build time. |
| Article source/content/type modules | Place these under `domains/articles/` to follow Term/UI's feature-owned domain structure. |
| `/articles`, `/articles/[slug]`, `/articles/category/[category]` | Keep these clean, descriptive, static routes under `app/(seo)/articles/`. |
| `/authors` and `/authors/[slug]` | Include both so author bylines resolve to useful, indexable profiles. |
| Featured article, spotlight, filters, category rail, cards | Recreate the information architecture and behavior with current Term/UI/shadcn styling. |
| TOC, sharing, reading time, excerpt, author card | Add only where it improves article use; use the generated MDX body/headings as the single content source. |
| Article, breadcrumb, collection, and Person JSON-LD | Extend existing helpers in `app/(seo)/_lib/json-ld.tsx`; serialize safely and keep every value aligned with visible page content. |
| Hand-made hero art or generated OG fallback | Use Term/UI visual tokens and real article titles; copy author avatar only for Max Wells' byline. |
| Rustify career/program CTA and PostHog events | Replace CTA with relevant Term/UI docs, widgets, GitHub, or installation paths. Use `shared/lib/events.ts` only for events its validated schema supports; no analytics vendor dependency. |

## 5. Content model and editorial workflow

### Article source and metadata

Add an `articles` Fumadocs collection for `content/articles/*.mdx`. Keep its schema small, explicit, and build-validated. Proposed required frontmatter:

```yaml
title: "Build Your First Rust TUI with Ratatui"
short_title_thumbnail: "First Ratatui App"
description: "Build a small interactive terminal app in Rust with Ratatui..."
category: "getting-started"
publish_date: "YYYY-MM-DD"
author: "Max Wells"
author_image: "/articles/author-max-wells.webp"
```

Optional fields: `last_updated`, `image`, `image_alt`, and a short editorial `keywords` list. Keep editorial target phrases in frontmatter only if useful for the team; never emit `<meta name="keywords">`. Google says that tag has no effect on indexing or ranking ([supported meta tags](https://developers.google.com/search/docs/crawling-indexing/special-tags)).

Start with a compact closed category enum: `getting-started`, `guides`, `widgets`, `engineering`, `comparisons`, and `design`. Assign one primary category per article. A category route becomes indexable only when it has at least three substantial articles and a useful category introduction; until then, its hub chip can filter `/articles` without linking to a thin archive page.

Requirements:

- `title`, `description`, `category`, `publish_date`, `author`, and thumbnail label must be non-empty and length-bounded in Zod.
- Category must be a closed union exported from dependency-free `domains/articles/categories.ts` so runtime and MDX build validation share one source of truth.
- Dates must parse as real calendar dates. `last_updated` changes only after a meaningful edit; do not stamp every build or typo fix.
- Slug comes from the MDX filename; filenames use lowercase kebab-case and remain stable after publication.
- Keep one authoritative author name per visible byline. Unknown authors fail validation or are explicitly represented in the author registry.
- Use a short, accurate description that summarizes the actual page. Do not keyword-stuff it.
- Use `image` only for authored visual assets. Without one, render a branded terminal/card thumbnail and generate a per-article social image.

### Article domain API

Expose a small, server-only content API modeled on RUSTIFY:

- `getAllArticles()`: complete sorted records for static generation/sitemap use.
- `getArticleSummaries()`: card and listing data without compiled MDX bodies.
- `getArticleBySlug(slug)`: article detail or `null`.
- Sort deterministically by `publish_date` descending; use slug as a stable tie-breaker.
- Reuse React `cache` or the framework's existing memoization pattern so hub, detail metadata, OG, and sitemap do not parse the content repeatedly.
- Derive excerpt, reading time, and heading IDs from the processed MDX text or generated page data. Avoid parallel handwritten copies of the article body.
- Keep all MDX rendering on the server. Use existing `mdxComponents`, code highlighter, shadcn components, and current docs typography before adding article-only components.

### Author model and Max Wells asset

- Make `Max Wells` the initial named byline, as requested, and create `/authors/max-wells`.
- Copy `public/articles/author-max-wells.webp` from the RUSTIFY reference repository into Term/UI's `public/articles/` when the author implementation begins. User has authorized this reuse.
- Reuse name/avatar; write a Term/UI-specific role and bio only from verified facts. Do not carry over Rustify career claims or social links as if they establish Ratatui experience. Add `Term/UI Contributors` for genuinely collaborative or unattributed pieces.
- Show author name/avatar on article pages and link to the author profile. Add `Person` JSON-LD only when matching information is visible on the profile page.
- Add `/authors` only when its content is useful; with one author it can simply list the one profile, without empty decorative cards.

### Editorial quality bar

Every published article must:

1. Solve one named reader task and own one primary search intent.
2. Start with the answer, then explain prerequisites and trade-offs.
3. Include tested Rust/Ratatui examples with exact crate versions and run instructions. The snippets must match the Ratatui API in the repository's current Rust lockfile/docs at publication time.
4. Include a meaningful result: terminal output/screenshot, interaction behavior, architecture diagram, tested code, or a comparison table based on stated criteria.
5. Link to relevant Term/UI pages and official upstream docs/crates. Prefer primary sources for API/version claims.
6. Explain limitations (terminal size, backend, platform, event model, color support) where relevant.
7. Carry a named author, publish date, and a genuine update date only when updated materially.
8. Be reviewed against current Google Search Console queries and the live SERP. Do not fabricate query volume or guarantee rankings.

Google's guidance favors useful, reliable, people-first pages grounded in first-hand knowledge; it warns against scaled pages that exist mainly to capture search variations ([people-first content](https://developers.google.com/search/docs/fundamentals/creating-helpful-content), [spam policies](https://developers.google.com/search/docs/essentials/spam-policies)). Fourteen briefs are a backlog, not a requirement to publish fourteen thin pages at once.

## 6. Initial SEO article backlog: 14 briefs

Keyword phrases below are intent hypotheses, not verified volume estimates. Validate them with Search Console, autocomplete, and current result pages before drafting. Keep the article title, H1, slug, outline, and internal links aligned to that one intent.

### 1. Build Your First Rust TUI with Ratatui

- **Primary intent:** `Ratatui tutorial`; beginner wants a working first app.
- **Supporting phrases:** `Rust TUI tutorial`, `build terminal UI Rust`, `Ratatui getting started`.
- **Slug:** `build-first-rust-tui-ratatui`.
- **Category:** `getting-started`.
- **Angle:** A small interactive task/status app from `cargo new` through terminal rendering; explain the minimum app state and draw loop.
- **Outline:** install/prerequisites; dependency setup; terminal init/restore; app state; layout and first widgets; event loop; run/resize/quit; next steps.
- **Proof:** tested code matching the locked Ratatui version; a real checked-in terminal screenshot/frame; clear copy-and-run instructions.
- **Internal links:** installation docs, widget catalog, Button, List, Text Input, form guide.
- **Avoid overlap:** keep it to a complete first vertical slice. Reserve dashboards, async, and detailed event architecture for their own guides.

### 2. Ratatui Architecture Explained: Widgets, App State, and the Draw Loop

- **Primary intent:** `how Ratatui works`; developer wants the mental model before choosing an architecture.
- **Supporting phrases:** `Ratatui render loop`, `Ratatui app state`, `Rust TUI architecture`.
- **Slug:** `ratatui-architecture-widgets-state-draw-loop`.
- **Category:** `engineering`.
- **Angle:** Explain immediate-mode rendering, persistent application state, frame rendering, and the separation between input and drawing.
- **Outline:** terminal backend and frame; render-on-each-frame model; app-owned state; widget composition; input/update/render cycle; common ownership/state mistakes.
- **Proof:** one small annotated architecture diagram and a compact code example contrasting state with render output.
- **Internal links:** first-app tutorial, event loop, stateful widgets, docs source/copy model.
- **Avoid overlap:** conceptual reference, not another beginner step-by-step project.

### 3. Build a Terminal Dashboard in Rust with Ratatui

- **Primary intent:** `Ratatui dashboard`; developer wants a dashboard implementation.
- **Supporting phrases:** `Rust terminal dashboard`, `TUI dashboard Rust`, `Ratatui charts table`.
- **Slug:** `build-rust-terminal-dashboard-ratatui`.
- **Category:** `guides`.
- **Angle:** Compose summary cards, a chart, a recent-events list, and a table around a coherent sample dataset.
- **Outline:** dashboard layout; reusable panel regions; metric/chart/table composition; data refresh boundary; resize behavior; final app and extensions.
- **Proof:** a real visual preview at wide and narrow terminal sizes; complete, tested example source.
- **Internal links:** chart docs, Card, Table, Sparkline, layout guide, chart gallery.
- **Avoid overlap:** show composition and dashboard information hierarchy; link out for each widget's API details.

### 4. Ratatui Layouts: Constraints, Flex, and Resizing

- **Primary intent:** `Ratatui layout`; developer needs reliable layout sizing.
- **Supporting phrases:** `Ratatui Layout constraints`, `Ratatui Flex`, `responsive terminal UI Rust`.
- **Slug:** `ratatui-layout-constraints-flex-resizing`.
- **Category:** `guides`.
- **Angle:** Practical layout recipes that adapt to terminal size without assuming a fixed number of columns or rows.
- **Outline:** `Layout` and constraints; fixed/percentage/min/max/ratio trade-offs; nested layouts; responsive breakpoints from terminal `Rect`; common overflow traps; testing small terminals.
- **Proof:** snippets for a header/body/footer and a two-panel layout, validated at multiple dimensions.
- **Internal links:** Resizable Layout, Split Pane, Tabs, dashboard, official Ratatui layout docs.
- **Avoid overlap:** cover layout primitives and resize strategy; leave widget implementation to widget articles.

### 5. Ratatui Tables: Selection, Sorting, and Search

- **Primary intent:** `Ratatui table`; developer needs an interactive data table.
- **Supporting phrases:** `Ratatui TableState`, `Rust TUI table selection`, `Ratatui table sorting`.
- **Slug:** `ratatui-table-selection-sorting-search`.
- **Category:** `widgets`.
- **Angle:** Build a keyboard-navigable table with app-owned selection, sorting, and a simple search/filter state.
- **Outline:** rows/columns; `TableState`; selection and scrolling; sort state; search integration; empty/long-cell states; testing.
- **Proof:** runnable Rust sample plus wide/narrow preview and meaningful keyboard behavior.
- **Internal links:** Table, Sortable Table, Search Input, List, keyboard event guide.
- **Avoid overlap:** focus exclusively on table workflows; don't duplicate generic input widget docs.

### 6. Build Ratatui Forms with Text Input, Validation, and Focus

- **Primary intent:** `Ratatui form`; developer wants form controls in a terminal.
- **Supporting phrases:** `Ratatui text input`, `Rust TUI form validation`, `Ratatui focus management`.
- **Slug:** `ratatui-forms-text-input-validation-focus`.
- **Category:** `widgets`.
- **Angle:** Compose Term/UI's copyable form controls into a useful form; explain that app state and validation remain app-owned.
- **Outline:** field state; labels/errors; focus order; typing and submit; validation; keyboard navigation; accessible state cues; integration example.
- **Proof:** tested input flow including invalid, valid, and focused states; show the exact source copied by the reader.
- **Internal links:** Text Input, Text Area, Checkbox, Select List, Radio Card, form composition guide.
- **Avoid overlap:** demonstrate composition and state ownership, not a catalog of every input widget.

### 7. Ratatui Charts: Plot Live Data in a Terminal

- **Primary intent:** `Ratatui charts`; developer wants to visualize data in a TUI.
- **Supporting phrases:** `Ratatui chart widget`, `Rust terminal graph`, `Ratatui sparkline`.
- **Slug:** `ratatui-charts-terminal-data-visualization`.
- **Category:** `widgets`.
- **Angle:** Choose among line, bar, area, pie, radar, and sparkline displays using honest terminal constraints and one coherent dataset.
- **Outline:** chart data model; axes/labels; each chart family's best fit; refresh/state boundary; narrow-terminal trade-offs; legend and tooltip design.
- **Proof:** examples based on actual current Term/UI chart APIs, not illustrative APIs that do not compile.
- **Internal links:** chart gallery and family pages, Sparkline, dashboard, progress widgets.
- **Avoid overlap:** answer when to use each chart and show composition; defer full API docs to widget pages.

### 8. Ratatui Input Handling: Keyboard, Mouse, and Event Loops

- **Primary intent:** `Ratatui keyboard input`; developer wants controls to respond to user input.
- **Supporting phrases:** `Ratatui event handling`, `crossterm key events`, `Ratatui mouse support`.
- **Slug:** `ratatui-keyboard-mouse-event-loop`.
- **Category:** `engineering`.
- **Angle:** Explain terminal setup, event reading, key mapping, app state updates, redraw decisions, and mouse-coordinate pitfalls.
- **Outline:** terminal lifecycle; event loop; key press/repeat/release distinctions supported by current APIs; map events to actions; mouse capture and coordinates; quit and restore reliably.
- **Proof:** runnable example with keyboard behavior, plus mouse behavior only where supported by the chosen backend.
- **Internal links:** first-app tutorial, Dialog, Drawer, Toast, Resizable Layout, official Crossterm docs.
- **Avoid overlap:** establish synchronous event handling; defer async channels and background tasks to article 9.

### 9. Async Ratatui with Tokio: Channels, Tasks, and Redraws

- **Primary intent:** `async Ratatui`; developer wants background work without freezing a terminal UI.
- **Supporting phrases:** `Ratatui Tokio`, `Rust TUI background task`, `async terminal UI`.
- **Slug:** `async-ratatui-tokio-background-tasks`.
- **Category:** `engineering`.
- **Angle:** Show a safe boundary between async workers and a synchronous terminal event/render loop, using channels and explicit refresh policy.
- **Outline:** when async is warranted; task ownership; channels; cancellation/shutdown; event-loop integration; redraw rate; error propagation; avoid blocking terminal I/O.
- **Proof:** a small live-status example with graceful exit and documented dependency/API versions.
- **Internal links:** architecture, event handling, dashboard, progress/loading widgets.
- **Avoid overlap:** don't re-teach the basic event loop; focus on concurrent data and lifecycle.

### 10. How to Test Ratatui Applications with TestBackend

- **Primary intent:** `test Ratatui app`; developer wants repeatable UI tests.
- **Supporting phrases:** `Ratatui TestBackend`, `Rust TUI unit testing`, `test terminal UI Rust`.
- **Slug:** `test-ratatui-apps-testbackend`.
- **Category:** `engineering`.
- **Angle:** Test application state transitions and rendered buffers without a real terminal, then identify where integration tests remain necessary.
- **Outline:** separate update/render; construct a test frame/backend; assert buffer cells; test selection/validation; snapshots and brittleness; terminal lifecycle integration tests.
- **Proof:** tests that run using repository-compatible crate versions; show exact commands and expected assertions.
- **Internal links:** Button/Text Input tests or source pages, architecture, forms, tables.
- **Avoid overlap:** distinguish pure render/state tests from browser tests for Term/UI's demos.

### 11. Ratatui vs Cursive: Choosing a Rust TUI Framework

- **Primary intent:** `Ratatui vs Cursive`; developer is evaluating Rust terminal UI frameworks.
- **Supporting phrases:** `Rust TUI frameworks`, `Ratatui alternatives`, `Cursive vs Ratatui`.
- **Slug:** `ratatui-vs-cursive-rust-tui`.
- **Category:** `comparisons`.
- **Angle:** Fair comparison by rendering model, widgets, state ownership, event integration, extensibility, maintenance, and learning curve.
- **Outline:** who each fits; accurate comparison table; minimal examples from both; ecosystem and version check; decision matrix; migration/caveats.
- **Proof:** verify versions, APIs, licenses, maintenance, and claims against each project's official repository/docs on publication day.
- **Internal links:** first-app tutorial, architecture, widget catalog, relevant upstream projects.
- **Avoid overlap:** compare Rust libraries only. Do not mislabel non-Rust tools like Bubble Tea as Rust alternatives or make unsupported performance claims.

### 12. Copy Ratatui Widget Source into Your Existing App

- **Primary intent:** `copy Ratatui widgets`; developer wants to add reusable source to an existing Rust app.
- **Supporting phrases:** `copyable Ratatui components`, `Ratatui component source`, `customize Ratatui widget`.
- **Slug:** `ratatui-widgets-copyable-components`.
- **Category:** `widgets`.
- **Angle:** Explain the trade-off between Ratatui primitives, ecosystem crates, and copyable app-owned widget source; show a composed screen using Term/UI.
- **Outline:** what Ratatui calls a widget; primitive vs composed component; selection criteria; copy/adapt workflow; ownership/theming; example composition; limitations and source inspection.
- **Proof:** source pages and copy commands that work in the current repository; clearly state that Term/UI provides source modules, not a Cargo dependency.
- **Internal links:** widget catalog, installation/copy instructions, Button, Card, Dialog, Table, GitHub.
- **Avoid overlap:** `/docs/widgets` owns the broad widget catalog/API discovery intent; this article owns the copy, adapt, and maintain component source workflow.

### 13. Terminal UI UX: Navigation, Focus, and Accessibility

- **Primary intent:** `terminal UI design`; developer wants a usable TUI interaction model.
- **Supporting phrases:** `TUI design patterns`, `terminal app keyboard navigation`, `accessible terminal UI`.
- **Slug:** `terminal-ui-design-navigation-focus-accessibility`.
- **Category:** `design`.
- **Angle:** Practical design rules for keyboard-first interaction, focus visibility, labels, errors, color, and reduced assumptions about terminal size.
- **Outline:** information hierarchy; navigation model; focus order and visible focus; keyboard discoverability; color contrast and non-color cues; empty/error/loading states; terminal dimensions.
- **Proof:** before/after screenshots or frames and a concrete accessibility review checklist; avoid unsupported claims about assistive technology behavior.
- **Internal links:** Help Screen, Menu, Tabs, Tooltip, Dialog, form guide, layout guide.
- **Avoid overlap:** UX patterns, not an API tutorial for each widget.

### 14. Shipping a Rust TUI Across Terminals and Platforms

- **Primary intent:** `cross-platform Rust TUI`; developer is preparing a terminal app for users.
- **Supporting phrases:** `Rust TUI unicode`, `terminal color support`, `Ratatui terminal resize`, `ship Rust terminal app`.
- **Slug:** `ship-cross-platform-rust-terminal-ui`.
- **Category:** `engineering`.
- **Angle:** A practical pre-release checklist for terminal capabilities, resize, Unicode width, color, alternate screen/raw mode cleanup, and packaging.
- **Outline:** terminal/backend compatibility; color and capability detection; Unicode/grapheme width; resize; cleanup on errors/panic; OS/terminal test matrix; binaries and release notes.
- **Proof:** tested platform matrix, cite upstream crates/terminal docs, and clearly separate verified behavior from recommendations.
- **Internal links:** event handling, layout, installation, docs changelog, GitHub.
- **Avoid overlap:** production readiness and portability, not a generic Cargo release guide.

## 7. Route, page, and interaction requirements

### Routes and discoverability

- `/articles`: indexable English hub with clear H1/introduction, featured article, a few spotlight cards, category browse links, and a crawlable server-rendered article list.
- `/articles/[slug]`: one static route per published article; unknown slugs return a real 404.
- `/articles/category/[category]`: static landing pages only for categories with at least three substantial articles and a useful introduction. Unknown, empty, or not-yet-indexable categories return 404; their hub filters can still work.
- `/authors` and `/authors/[slug]`: useful byline index/profile pages; every byline links to its profile.
- Add at least one normal HTML link to `/articles` from existing Term/UI navigation, footer, or a relevant landing page. Add contextual links from docs and related articles. Don't rely on sitemap discovery alone.
- Use category filter query parameters only as a usability aid. Ignore invalid category values and apply `noindex,follow` to parameterized hub views while retaining the clean `/articles` canonical.

### Hub and category UX

- Recreate RUSTIFY's featured/spotlight, searchable/filterable list, category browse rail, counts, and card thumbnails, with Term/UI colors and responsive layout.
- Keep category and article links present in server-rendered HTML. Client-side filtering may enhance the listing, but must not be the only way crawlers or keyboard users can reach content.
- Treat categories as a compact, descriptive closed set. Do not generate indexable empty or thin category pages; require three substantial articles plus a useful introduction before indexing an archive.
- Add only validated tracking events to the existing `shared/lib/events.ts` schema; current policy sends no activity to a hosting vendor.

### Article detail UX

- Show category breadcrumbs, H1, author, publish/update dates, reading time, and a clear hero/terminal visual.
- Render compiled MDX with syntax highlighting, copyable code where the existing code component supports it, and accessible callouts/tables/figures.
- Provide a TOC from real H2 headings, with stable heading IDs and usable keyboard/focus behavior. Keep the TOC out of the way on small screens.
- Add article sharing only if it reuses a small existing mechanism; do not add a sharing package for a single article section.
- End with a contextual next step into Term/UI docs/widgets/GitHub, not Rustify career funnels or unrelated sponsorship.
- Use descriptive image alt text; decorative terminal screenshots use empty alt only when the surrounding copy already conveys their information.

## 8. Technical SEO and structured data

### Metadata and canonical URLs

- Use the existing `createPageMetadata` in `app/(seo)/_lib/metadata.ts` for unique title, description, canonical, Open Graph, and Twitter metadata.
- Article metadata uses its true canonical path, article social type, published/modified dates, and the author name. Set all nested Open Graph/Twitter fields deliberately; Next.js metadata objects do not always deep-merge nested objects as authors expect ([Next.js metadata API](https://nextjs.org/docs/app/api-reference/functions/generate-metadata)).
- Use `generateMetadata` for slug-dependent articles. Return the page-specific generated OG image or a real authored image, with dimensions and useful alt text.
- Canonical URLs use `https://termui.rustify.app`, with one trailing-slash policy consistent across metadata, internal links, sitemap, and redirects. Canonical tags and sitemap entries must agree ([Google canonicalization](https://developers.google.com/search/docs/crawling-indexing/canonicalization)).
- Do not add keyword meta tags or try to rank each filter/sort permutation.

### Structured data

- Article detail: `Article` or `BlogPosting` with visible headline, description, canonical URL, representative absolute image, accurate `datePublished`/`dateModified`, author entity, publisher, and main page identity. Link author to `/authors/[slug]` where known. Follow Google's author properties guidance and test actual output ([Article structured data](https://developers.google.com/search/docs/appearance/structured-data/article)).
- Add `BreadcrumbList` matching visible breadcrumbs for article, category, and author pages, reusing the existing helper.
- Hub: `CollectionPage` plus an `ItemList` only when entries visibly link to those articles. Category pages may use the same pattern if it remains accurate and useful.
- Author profiles: `Person` with only visible, verified name, role, profile URL, image, and `sameAs` social URLs.
- Do not add `FAQPage` expecting rich results: Google limits FAQ rich results to well-known authoritative government and health sites ([FAQ/HowTo changes](https://developers.google.com/search/blog/2023/08/howto-faq-changes)). An on-page FAQ is optional only when it helps readers.
- No fake review, rating, `HowTo`, `Product`, or performance data. Structured data must match user-visible content.
- Validate representative output with Schema.org/Google tools and URL Inspection; structured data helps parsing, but does not guarantee a special result.

### Sitemap, crawl, and index policy

- Extend `app/(seo)/sitemap.ts`; preserve existing home/docs/chart entries.
- Include only canonical, published article URLs, editorially useful category URLs, `/articles`, and valid author URLs. Use actual `last_updated` or `publish_date`; never current build time.
- Keep query-string filters out of the sitemap. Do not include draft, preview, empty-category, or duplicate routes.
- Keep `robots.txt` public behavior unchanged and ensure its sitemap URL remains the canonical Term/UI sitemap. Sitemap submission is a discovery hint, not an indexing guarantee ([sitemap guidance](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)).
- New article routes must be statically rendered at build time when the content is local and immutable; keep route generation bounded to the validated MDX set.

### Open Graph images

- Add route-level generated OG images for the hub and article slug, using Next.js `ImageResponse` and the existing Term/UI brand visual system ([Next.js OG image convention](https://nextjs.org/docs/app/api-reference/file-conventions/metadata/opengraph-image)).
- Use 1200x630, readable short title, Term/UI mark, restrained terminal motif, and descriptive alt. Test long titles, apostrophes, Unicode, font loading, and unknown slugs.
- If an article has approved hero art, use it consistently in the page, metadata, and JSON-LD. Otherwise use generated OG art; do not accidentally let the global fallback override it.

## 9. Implementation phases and acceptance criteria

### Phase 0 — Validate editorial targets

- [ ] Review Search Console queries/pages for Ratatui, Rust TUI, terminal UI, and current widget docs.
- [ ] Check live SERPs and query phrasing for the 14 briefs; record one intent and one primary page per cluster.
- [ ] Check current docs/routes for overlap; adjust or drop briefs that would duplicate an existing page.
- [ ] Choose first release batch based on demand and ability to produce verified examples; do not publish thin drafts to hit a count.

**Done when:** briefs have non-overlapping intents, named reader tasks, and a realistic evidence/source plan.

### Phase 1 — Data and MDX foundation

- [ ] Add article/category/author types and closed taxonomy under `domains/articles/`.
- [ ] Extend `source.config.ts` with the strict Fumadocs `articles` collection for `content/articles/`.
- [ ] Implement source loader and cached content helpers for summaries, detail, slugs, excerpts, headings, and reading time.
- [ ] Validate filename slugs, dates, author mapping, descriptions, category values, image paths, and duplicate data during build.
- [ ] Add one short fixture article only when required to build the foundation; do not ship placeholder content.
- [ ] Copy the authorized Max Wells portrait into `public/articles/` and verify dimensions/format; add the real profile data only after factual fields are reviewed.

**Done when:** MDX content builds through the existing toolchain, invalid frontmatter fails clearly, and summaries/details share one source of truth.

### Phase 2 — Public article routes and site shell

- [ ] Add the `(seo)` layout and article layout without duplicate root/site header/footer output.
- [ ] Add article hub, static slug detail route, populated category routes, author index/profile routes, and 404 behavior.
- [ ] Render initial hub content and category/article links in server HTML; implement filter/search enhancement with invalid query handling and `noindex,follow` metadata.
- [ ] Reuse shadcn and current Term/UI components. Match existing light/dark theme and responsive behavior.
- [ ] Add the article hub to a visible site entry point and add contextual links from relevant docs pages.

**Done when:** direct URL loads, navigation, browser refresh, narrow view, keyboard use, unknown slug/category, and static production rendering behave correctly.

### Phase 3 — Article reading experience

- [ ] Add byline/profile link, accurate date labels, reading time, category breadcrumbs, H2 TOC, share links, code copy, and responsive MDX typography.
- [ ] Add relevant bottom-of-article links/CTA to Term/UI docs, widgets, installation, or GitHub.
- [ ] Add `Max Wells` profile with the approved avatar and verified Term/UI-specific role/bio/social URLs.
- [ ] Add `/authors` only with meaningful profile cards and article counts; no fabricated team roster.

**Done when:** articles are readable, navigable, copyable, and clearly authored on desktop/mobile and keyboard-only.

### Phase 4 — SEO, discovery, and social cards

- [ ] Generate unique metadata and self-canonicals for hub, category, article, and author routes.
- [ ] Add Article/BlogPosting, BreadcrumbList, CollectionPage/ItemList, and Person JSON-LD where content supports them.
- [ ] Add hub and per-article generated OG images plus optional real hero-image support.
- [ ] Extend sitemap using real publication/update dates and only populated canonical URLs.
- [ ] Audit `robots.txt`, canonical origin, image URLs, metadata on production domain, and internal links.

**Done when:** no duplicate canonical targets, missing metadata, broken assets, invalid JSON-LD, or query-param sitemap URLs remain.

### Phase 5 — Publish article batches

- [ ] Draft and technically review articles in order of query evidence and source readiness.
- [ ] Compile/run every code example against repository-compatible Ratatui versions.
- [ ] Add the first 3–4 finished, distinct pieces, then inspect reader feedback and Search Console before continuing.
- [ ] Publish remaining backlog in useful clusters, linking each piece to the matching guide, widget, and related articles.
- [ ] Revisit `last_updated` only when code/API, upstream facts, or substantial editorial guidance changes.

**Done when:** all selected launch articles meet the quality bar; the rest remain clearly marked editorial backlog until ready.

### Phase 6 — Release and ongoing measurement

- [ ] Run repository `pnpm check:quality` and production `pnpm build:production`; verify articles do not require Rust demo generation/toolchain unless an article asset actually does.
- [ ] Inspect generated HTML for title, description, canonical, visible text, links, OG/Twitter tags, and JSON-LD.
- [ ] Verify a representative article, hub, category, author page, sitemap, and robots response on the production deployment.
- [ ] Submit/update the sitemap in Search Console and monitor indexing, impressions, clicks, CTR, and query/page match.
- [ ] Review articles after Ratatui or backend/API changes; remove or redirect retired slugs rather than silently changing published URLs.

**Done when:** deploy has clean quality gates, production routes work, and Search Console has the expected canonical URLs available for inspection.

## 10. Acceptance checklist

### Editorial

- [ ] Every published piece answers one distinct intent and has a reader-useful result.
- [ ] All snippets compile/run against declared versions; upstream API claims link to primary sources.
- [ ] Article content is original, reviewed, accurate, and in English; no unsupported numbers or generic filler.
- [ ] Articles link to relevant Term/UI docs and to related articles; docs link back where useful.

### Product and implementation

- [ ] Term/UI brand/domain and shadcn/theme behavior preserved; no Rustify career CTA or sponsor surface appears.
- [ ] Articles remain MDX in `content/articles/`; docs and changelog sources remain where they are.
- [ ] Zod catches bad metadata/categories/author/image paths at build time.
- [ ] Hub, detail, populated category, and author pages are statically generated and return correct 404s.
- [ ] Existing site header/footer appear exactly once on every article/author route.
- [ ] Keyboard navigation, reduced viewport widths, code copy, and image alt behavior work.

### Search and sharing

- [ ] Each public route has a unique title/description and correct self-canonical on `termui.rustify.app`.
- [ ] Filter query views cannot become duplicate indexable pages.
- [ ] Sitemap lists canonical published articles, non-empty categories, and valid author pages with honest dates.
- [ ] JSON-LD validates and exactly reflects visible author/date/title/breadcrumb data.
- [ ] OG image previews are readable and correct for the hub and representative long/short article titles.
- [ ] No `meta keywords`, empty category pages, fake freshness, thin query variants, or fabricated author expertise.

## 11. Primary references

### Local implementation references

- RUSTIFY article implementation: `/Users/user/dev/1-RUST/RUSTIFY-APP/app_v2_website/{source.config.ts,src/domain/articles,src/lib/authors.ts,src/lib/seo.ts,app/(seo)/articles,app/(seo)/authors,app/sitemap.ts,public/articles}`.
- Term/UI content foundation: `source.config.ts`, `domains/docs/source.ts`, `domains/docs/docs.ts`, `app/(seo)/sitemap.ts`, `app/(seo)/_lib/metadata.ts`, `app/(seo)/_lib/json-ld.tsx`, `shared/config/routes.ts`, `shared/lib/events.ts`.

### Current primary technical guidance

- [Google Search Essentials](https://developers.google.com/search/docs/essentials)
- [People-first content](https://developers.google.com/search/docs/fundamentals/creating-helpful-content)
- [Article structured data](https://developers.google.com/search/docs/appearance/structured-data/article)
- [Canonicalization](https://developers.google.com/search/docs/crawling-indexing/canonicalization)
- [Sitemap best practices](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)
- [Supported and unsupported meta tags](https://developers.google.com/search/docs/crawling-indexing/special-tags)
- [FAQ and HowTo rich result changes](https://developers.google.com/search/blog/2023/08/howto-faq-changes)
- [Next.js Metadata API](https://nextjs.org/docs/app/api-reference/functions/generate-metadata)
- [Next.js generated Open Graph images](https://nextjs.org/docs/app/api-reference/file-conventions/metadata/opengraph-image)
