# Term/UI Articles and SEO Plan

**Status:** Proposed implementation plan
**Prepared:** 2026-09-21
**Product:** Term/UI
**Canonical origin:** `https://termui.rustify.app`

## 1. Objective

Build a complete, maintainable English article section for developers searching for Ratatui, Rust terminal UI, Rust TUI widgets, and practical terminal application guidance. Reuse the article system and editorial workflow from `RUSTIFY-APP/app_v2_website`, adapted to Term/UI's existing Next.js, Fumadocs, domain layout, shadcn UI, Ratatui docs, and independent product identity.

The deliverable is a useful article library, not a set of keyword landing pages. Each article must solve a distinct task, show working Rust examples, and connect readers to the matching Term/UI documentation and copyable widget source.

This plan defines the system and a coordinated launch of 12 articles. Finish and review the full cohort before publishing all articles, category pages, and author pages together in one production release. It does not claim traffic/search volume; validate the 12 query intents against Google Search Console and current search results before drafting.

## 2. Guardrails

- Write the site, article, metadata, and UI copy in English.
- Use `termui.rustify.app` for all canonical and absolute URLs. Keep Term/UI an independent product; do not inherit Rustify product CTAs or branding.
- Preserve Term/UI's existing shadcn UI, site shell, theme behavior, and visual conventions. Reuse RUSTIFY's information architecture and behavior; adapt the presentation to Term/UI.
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
| Featured article, spotlight, searchable/category-filtered list, Load More, category rail, cards | Recreate the full hub flow with current Term/UI/shadcn styling; keep links server-rendered and select featured/spotlight entries explicitly. |
| Sticky TOC/share sidebar, reading time, excerpt, author card, closing CTA | Include the full article-reading flow, using generated MDX headings and Term/UI destinations. |
| FAQ extraction and accordion | Support optional visible, accessible FAQs when they answer real reader questions; do not add FAQ rich-result markup. |
| Author index/profile pages, social links, article counts | Include substantive, indexable author pages with verified profile details and authored articles. |
| Article, breadcrumb, collection, and Person JSON-LD | Extend existing helpers in `app/(seo)/_lib/json-ld.tsx`; serialize safely and keep every value aligned with visible page content. |
| Hand-made hero art or generated OG fallback | Use Term/UI visual tokens and real article titles; copy author avatar only for Max Wells' byline. |
| Rustify career/program CTA and analytics integration | Replace CTA with relevant Term/UI docs, widgets, GitHub, or installation paths. Add no product analytics or tracking; Search Console is only for SEO search performance. |

## 5. Content model and editorial workflow

### Article source and metadata

Add an `articles` Fumadocs collection for `content/articles/*.mdx`. Keep its schema small, explicit, and build-validated. Proposed required frontmatter:

```yaml
title: "Build Your First Rust TUI with Ratatui"
short_title_thumbnail: "First Ratatui App"
description: "Build a small interactive terminal app in Rust with Ratatui..."
keywords: ["Ratatui tutorial", "Rust TUI"]
category: "building"
order: 1
publish_date: "YYYY-MM-DD"
author: "Max Wells"
author_image: "/articles/author-max-wells.webp"
```

Optional fields: `keywords`, `last_updated`, `image`, and `image_alt`. `keywords` defaults to an empty array and remains editorial data; never emit `<meta name="keywords">`. Google says that tag has no effect on indexing or ranking ([supported meta tags](https://developers.google.com/search/docs/crawling-indexing/special-tags)). `order` sets deliberate hub/spotlight order for articles sharing the coordinated launch date.

Use a compact closed category enum: `building`, `widgets`, `engineering`, and `production`. Assign exactly three distinct launch articles to each category. Every category page gets a unique, useful introduction and curated article list, so all four category pages can be indexable at launch.

Assign `order: 1..12` in the sequence below; article 1 is featured, articles 2–5 are the initial spotlight, and the remaining list follows that explicit order. After launch, sort new pieces by publication date and then order.

Requirements:

- `title`, `description`, `category`, `publish_date`, `author`, `short_title_thumbnail`, and `author_image` must be non-empty and length-bounded in Zod; when present, validate `keywords` as a string array. Validate `order` as a positive integer.
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
- Sort deterministically by `publish_date` descending, `order` ascending, then slug. This makes the featured and spotlight selection intentional when all launch articles share one publication date.
- Reuse React `cache` or the framework's existing memoization pattern so hub, detail metadata, OG, and sitemap do not parse the content repeatedly.
- Derive excerpt, reading time, and heading IDs from the processed MDX text or generated page data. Avoid parallel handwritten copies of the article body.
- Keep all MDX rendering on the server. Use existing `mdxComponents`, code highlighter, shadcn components, and current docs typography before adding article-only components.

### Author model and Max Wells asset

- Make `Max Wells` the initial named byline, as requested, and create `/authors/max-wells`.
- Copy `public/articles/author-max-wells.webp` from the RUSTIFY reference repository into Term/UI's `public/articles/` when the author implementation begins. User has authorized this reuse.
- Reuse name/avatar; write a Term/UI-specific bio from verified facts. Do not carry over Rustify career claims or social links as if they establish Ratatui experience. Describe the author's actual Term/UI editorial work where personal credentials are unavailable. Add `Term/UI Contributors` only for genuinely collaborative or unattributed pieces.
- Show author name/avatar on article pages and link to the author profile. Add `Person` JSON-LD only when matching information is visible on the profile page.
- Make `/authors` a useful index even with one named author: explain editorial focus/standards and link to the profile. Keep the profile indexable with a useful factual bio and authored-article archive; show personal role, credentials, portrait, or social links only where verified. Never invent expertise to make the page seem substantial.

### Article UX parity details

- Article hub: featured article, spotlight rows, search, multi-select category filters, result count, Load More, and category browse rail/cards. Curate featured/spotlight with the `order` field, not incidental file order.
- Article detail: sticky H2 table of contents and share row on wide screens; usable compact versions on mobile; include Copy Link and appropriate social share links, plus reading time, excerpt/hero, author profile link, breadcrumb, and a contextual closing CTA.
- Category page: breadcrumb, unique introduction, curated cards, article count, and related next steps. All four launch category pages receive self-canonicals and indexable metadata.
- FAQ: optional accessible accordion rendered from authored MDX FAQ content. Extract items only if needed for the UI. Do not emit `FAQPage` JSON-LD or promise FAQ rich results.
- Analytics: no PostHog, tracking scripts, reader-event collection, or other analytics vendor for this site at launch. Use Search Console only for SEO search performance.

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

Google's guidance favors useful, reliable, people-first pages grounded in first-hand knowledge; it warns against scaled pages that exist mainly to capture search variations ([people-first content](https://developers.google.com/search/docs/fundamentals/creating-helpful-content), [spam policies](https://developers.google.com/search/docs/essentials/spam-policies)). Publish the 12-piece cohort only when every article and category page meets that quality bar; do not release partial batches.

## 6. Coordinated SEO launch: 12 article briefs

Keyword phrases below are intent hypotheses, not verified volume estimates. Validate them with Search Console, autocomplete, and current result pages before drafting. Keep the article title, H1, slug, outline, and internal links aligned to that one intent. Launch the complete set together; every article must be ready before the release date.

| Category | Launch articles | Count |
| --- | --- | ---: |
| Building | First Rust TUI, terminal dashboard, Ratatui layouts | 3 |
| Widgets | Tables, forms, charts | 3 |
| Engineering | Input/event loop, async with Tokio, TestBackend testing | 3 |
| Production | Framework comparison, terminal UX, cross-platform release | 3 |

### 1. Build Your First Rust TUI with Ratatui

- **Primary intent:** `Ratatui tutorial`; beginner wants a working first app.
- **Supporting phrases:** `Rust TUI tutorial`, `build terminal UI Rust`, `Ratatui getting started`.
- **Slug:** `build-first-rust-tui-ratatui`.
- **Category:** `building`.
- **Angle:** A small interactive task/status app from `cargo new` through terminal rendering; explain the minimum app state and draw loop.
- **Outline:** install/prerequisites; dependency setup; terminal init/restore; app state; layout and first widgets; event loop; run/resize/quit; next steps.
- **Proof:** tested code matching the locked Ratatui version; a real checked-in terminal screenshot/frame; clear copy-and-run instructions.
- **Internal links:** installation docs, widget catalog, Button, List, Text Input, form article.
- **Avoid overlap:** keep it to a complete first vertical slice. Reserve dashboards, async, and detailed event architecture for their own guides.

### 2. Build a Terminal Dashboard in Rust with Ratatui

- **Primary intent:** `Ratatui dashboard`; developer wants a dashboard implementation.
- **Supporting phrases:** `Rust terminal dashboard`, `TUI dashboard Rust`, `Ratatui charts table`.
- **Slug:** `build-rust-terminal-dashboard-ratatui`.
- **Category:** `building`.
- **Angle:** Compose summary cards, a chart, a recent-events list, and a table around a coherent sample dataset.
- **Outline:** dashboard layout; reusable panel regions; metric/chart/table composition; data refresh boundary; resize behavior; final app and extensions.
- **Proof:** a real visual preview at wide and narrow terminal sizes; complete, tested example source.
- **Internal links:** chart docs, Card, Table, Sparkline, layout article, chart gallery.
- **Avoid overlap:** show composition and dashboard information hierarchy; link out for each widget's API details.

### 3. Ratatui Layouts: Constraints, Flex, and Resizing

- **Primary intent:** `Ratatui layout`; developer needs reliable layout sizing.
- **Supporting phrases:** `Ratatui Layout constraints`, `Ratatui Flex`, `responsive terminal UI Rust`.
- **Slug:** `ratatui-layout-constraints-flex-resizing`.
- **Category:** `building`.
- **Angle:** Practical layout recipes that adapt to terminal size without assuming a fixed number of columns or rows.
- **Outline:** `Layout` and constraints; fixed/percentage/min/max/ratio trade-offs; nested layouts; responsive breakpoints from terminal `Rect`; common overflow traps; testing small terminals.
- **Proof:** snippets for a header/body/footer and a two-panel layout, validated at multiple dimensions.
- **Internal links:** Resizable Layout, Split Pane, Tabs, dashboard, official Ratatui layout docs.
- **Avoid overlap:** cover layout primitives and resize strategy; leave widget implementation to widget articles.

### 4. Ratatui Tables: Selection, Sorting, and Search

- **Primary intent:** `Ratatui table`; developer needs an interactive data table.
- **Supporting phrases:** `Ratatui TableState`, `Rust TUI table selection`, `Ratatui table sorting`.
- **Slug:** `ratatui-table-selection-sorting-search`.
- **Category:** `widgets`.
- **Angle:** Build a keyboard-navigable table with app-owned selection, sorting, and a simple search/filter state.
- **Outline:** rows/columns; `TableState`; selection and scrolling; sort state; search integration; empty/long-cell states; testing.
- **Proof:** runnable Rust sample plus wide/narrow preview and meaningful keyboard behavior.
- **Internal links:** Table, Sortable Table, Search Input, List, keyboard event article.
- **Avoid overlap:** focus exclusively on table workflows; don't duplicate generic input widget docs.

### 5. Build Ratatui Forms with Text Input, Validation, and Focus

- **Primary intent:** `Ratatui form`; developer wants form controls in a terminal.
- **Supporting phrases:** `Ratatui text input`, `Rust TUI form validation`, `Ratatui focus management`.
- **Slug:** `ratatui-forms-text-input-validation-focus`.
- **Category:** `widgets`.
- **Angle:** Compose Term/UI's copyable form controls into a useful form; explain that app state and validation remain app-owned.
- **Outline:** field state; labels/errors; focus order; typing and submit; validation; keyboard navigation; accessible state cues; integration example.
- **Proof:** tested input flow including invalid, valid, and focused states; show the exact source copied by the reader.
- **Internal links:** Text Input, Text Area, Checkbox, Select List, Radio Card, form composition article.
- **Avoid overlap:** demonstrate composition and state ownership, not a catalog of every input widget.

### 6. Ratatui Charts: Plot Live Data in a Terminal

- **Primary intent:** `Ratatui charts`; developer wants to visualize data in a TUI.
- **Supporting phrases:** `Ratatui chart widget`, `Rust terminal graph`, `Ratatui sparkline`.
- **Slug:** `ratatui-charts-terminal-data-visualization`.
- **Category:** `widgets`.
- **Angle:** Choose among line, bar, area, pie, radar, and sparkline displays using honest terminal constraints and one coherent dataset.
- **Outline:** chart data model; axes/labels; each chart family's best fit; refresh/state boundary; narrow-terminal trade-offs; legend and tooltip design.
- **Proof:** examples based on actual current Term/UI chart APIs, not illustrative APIs that do not compile.
- **Internal links:** chart gallery and family pages, Sparkline, dashboard, progress widgets.
- **Avoid overlap:** answer when to use each chart and show composition; defer full API docs to widget pages.

### 7. Ratatui Input Handling: Keyboard, Mouse, and Event Loops

- **Primary intent:** `Ratatui keyboard input`; developer wants controls to respond to user input.
- **Supporting phrases:** `Ratatui event handling`, `crossterm key events`, `Ratatui mouse support`.
- **Slug:** `ratatui-keyboard-mouse-event-loop`.
- **Category:** `engineering`.
- **Angle:** Explain terminal setup, event reading, key mapping, app state updates, redraw decisions, and mouse-coordinate pitfalls.
- **Outline:** terminal lifecycle; event loop; key press/repeat/release distinctions supported by current APIs; map events to actions; mouse capture and coordinates; quit and restore reliably.
- **Proof:** runnable example with keyboard behavior, plus mouse behavior only where supported by the chosen backend.
- **Internal links:** first-app tutorial, Dialog, Drawer, Toast, Resizable Layout, official Crossterm docs.
- **Avoid overlap:** establish synchronous event handling; defer async channels and background tasks to article 8.

### 8. Async Ratatui with Tokio: Channels, Tasks, and Redraws

- **Primary intent:** `async Ratatui`; developer wants background work without freezing a terminal UI.
- **Supporting phrases:** `Ratatui Tokio`, `Rust TUI background task`, `async terminal UI`.
- **Slug:** `async-ratatui-tokio-background-tasks`.
- **Category:** `engineering`.
- **Angle:** Show a safe boundary between async workers and a synchronous terminal event/render loop, using channels and explicit refresh policy.
- **Outline:** when async is warranted; task ownership; channels; cancellation/shutdown; event-loop integration; redraw rate; error propagation; avoid blocking terminal I/O.
- **Proof:** a small live-status example with graceful exit and documented dependency/API versions.
- **Internal links:** first-app tutorial, event handling, dashboard, progress/loading widgets.
- **Avoid overlap:** don't re-teach the basic event loop; focus on concurrent data and lifecycle.

### 9. How to Test Ratatui Applications with TestBackend

- **Primary intent:** `test Ratatui app`; developer wants repeatable UI tests.
- **Supporting phrases:** `Ratatui TestBackend`, `Rust TUI unit testing`, `test terminal UI Rust`.
- **Slug:** `test-ratatui-apps-testbackend`.
- **Category:** `engineering`.
- **Angle:** Test application state transitions and rendered buffers without a real terminal, then identify where integration tests remain necessary.
- **Outline:** separate update/render; construct a test frame/backend; assert buffer cells; test selection/validation; snapshots and brittleness; terminal lifecycle integration tests.
- **Proof:** tests that run using repository-compatible crate versions; show exact commands and expected assertions.
- **Internal links:** Button/Text Input tests or source pages, first-app tutorial, forms, tables.
- **Avoid overlap:** distinguish pure render/state tests from browser tests for Term/UI's demos.

### 10. Ratatui vs Cursive: Choosing a Rust TUI Framework

- **Primary intent:** `Ratatui vs Cursive`; developer is evaluating Rust terminal UI frameworks.
- **Supporting phrases:** `Rust TUI frameworks`, `Ratatui alternatives`, `Cursive vs Ratatui`.
- **Slug:** `ratatui-vs-cursive-rust-tui`.
- **Category:** `production`.
- **Angle:** Fair comparison by rendering model, widgets, state ownership, event integration, extensibility, maintenance, and learning curve.
- **Outline:** who each fits; accurate comparison table; minimal examples from both; ecosystem and version check; decision matrix; migration/caveats.
- **Proof:** verify versions, APIs, licenses, maintenance, and claims against each project's official repository/docs on publication day.
- **Internal links:** first-app tutorial, widget catalog, relevant upstream projects.
- **Avoid overlap:** compare Rust libraries only. Do not mislabel non-Rust tools like Bubble Tea as Rust alternatives or make unsupported performance claims.

### 11. Terminal UI UX: Navigation, Focus, and Accessibility

- **Primary intent:** `terminal UI design`; developer wants a usable TUI interaction model.
- **Supporting phrases:** `TUI design patterns`, `terminal app keyboard navigation`, `accessible terminal UI`.
- **Slug:** `terminal-ui-design-navigation-focus-accessibility`.
- **Category:** `production`.
- **Angle:** Practical design rules for keyboard-first interaction, focus visibility, labels, errors, color, and reduced assumptions about terminal size.
- **Outline:** information hierarchy; navigation model; focus order and visible focus; keyboard discoverability; color contrast and non-color cues; empty/error/loading states; terminal dimensions.
- **Proof:** before/after screenshots or frames and a concrete accessibility review checklist; avoid unsupported claims about assistive technology behavior.
- **Internal links:** Help Screen, Menu, Tabs, Tooltip, Dialog, form article, layout article.
- **Avoid overlap:** UX patterns, not an API tutorial for each widget.

### 12. Shipping a Rust TUI Across Terminals and Platforms

- **Primary intent:** `cross-platform Rust TUI`; developer is preparing a terminal app for users.
- **Supporting phrases:** `Rust TUI unicode`, `terminal color support`, `Ratatui terminal resize`, `ship Rust terminal app`.
- **Slug:** `ship-cross-platform-rust-terminal-ui`.
- **Category:** `production`.
- **Angle:** A practical pre-release checklist for terminal capabilities, resize, Unicode width, color, alternate screen/raw mode cleanup, and packaging.
- **Outline:** terminal/backend compatibility; color and capability detection; Unicode/grapheme width; resize; cleanup on errors/panic; OS/terminal test matrix; binaries and release notes.
- **Proof:** tested platform matrix, cite upstream crates/terminal docs, and clearly separate verified behavior from recommendations.
- **Internal links:** event handling, layout, installation, docs changelog, GitHub.
- **Avoid overlap:** production readiness and portability, not a generic Cargo release guide.

## 7. Route, page, and interaction requirements

### Routes and discoverability

- `/articles`: indexable English hub with clear H1/introduction, featured article, a few spotlight cards, category browse links, and a crawlable server-rendered article list.
- `/articles/[slug]`: one static route per published article; unknown slugs return a real 404.
- `/articles/category/[category]`: four static category landing pages, each with three launch articles and a distinct, useful introduction. All four are indexable.
- `/authors` and `/authors/[slug]`: substantive, indexable byline index/profile pages; every byline links to its profile.
- All public editorial routes (`/articles`, all 12 article pages, all four category pages, `/authors`, and valid author profiles) ship with `index,follow`, self-canonicals, crawlable HTML links, and sitemap entries. Google chooses final index inclusion; the implementation must not block these pages with `noindex`, robots rules, or missing links.
- Add at least one normal HTML link to `/articles` from existing Term/UI navigation, footer, or a relevant landing page. Add contextual links from docs and related articles. Don't rely on sitemap discovery alone.
- Use category filter query parameters only as a usability aid. Ignore invalid category values and apply `noindex,follow` to parameterized hub views while retaining the clean `/articles` canonical.

### Hub and category UX

- Recreate RUSTIFY's featured/spotlight, searchable/filterable list, category browse rail, counts, and card thumbnails, with Term/UI colors and responsive layout.
- Keep category and article links present in server-rendered HTML. Client-side filtering may enhance the listing, but must not be the only way crawlers or keyboard users can reach content.
- Treat the four categories as a compact, descriptive closed set. Give each exactly three launch articles and a category-specific introduction with useful reading order, so every category archive has real standalone value and is indexable at launch.
- Do not add PostHog, tracking scripts, reader-event collection, or another analytics vendor for this site at launch. Search Console is for SEO search-performance reporting only.

### Article detail UX

- Show category breadcrumbs, H1, author, publish/update dates, reading time, and a clear hero/terminal visual.
- Render compiled MDX with syntax highlighting, copyable code where the existing code component supports it, and accessible callouts/tables/figures.
- Provide a TOC from real H2 headings, with stable heading IDs and usable keyboard/focus behavior. Keep the TOC out of the way on small screens.
- Add a small share row with Copy Link and plain social-share links; use browser APIs/URLs and existing components, with no tracking or sharing dependency.
- End with a contextual next step into Term/UI docs/widgets/GitHub, not Rustify career funnels or unrelated sponsorship.
- Use descriptive image alt text; decorative terminal screenshots use empty alt only when the surrounding copy already conveys their information.

## 8. Technical SEO and structured data

### Metadata and canonical URLs

- Use the existing `createPageMetadata` in `app/(seo)/_lib/metadata.ts` for unique title, description, canonical, Open Graph, and Twitter metadata.
- Article metadata uses its true canonical path, article social type, published/modified dates, and the author name. Set all nested Open Graph/Twitter fields deliberately; Next.js metadata objects do not always deep-merge nested objects as authors expect ([Next.js metadata API](https://nextjs.org/docs/app/api-reference/functions/generate-metadata)).
- Give all public editorial routes `index,follow`; do not apply `noindex` to any of the 12 articles, four category archives, hub, or valid author profiles. Query-parameter filter states remain non-editorial duplicates: point them to the clean hub canonical and keep them out of the sitemap.
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
- Include all 12 canonical article URLs, all four category URLs, `/articles`, `/authors`, and valid author profiles. Use actual `last_updated` or `publish_date`; never current build time.
- Keep query-string filters out of the sitemap. Do not include draft, preview, empty-category, or duplicate routes.
- Keep `robots.txt` public behavior unchanged and ensure its sitemap URL remains the canonical Term/UI sitemap. Sitemap submission is a discovery hint, not an indexing guarantee ([sitemap guidance](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)).
- New article routes must be statically rendered at build time when the content is local and immutable; keep route generation bounded to the 12 validated MDX files and four known categories.

### Open Graph images

- Add route-level generated OG images for the hub, categories, author profiles, and article slugs, using Next.js `ImageResponse` and the existing Term/UI brand visual system ([Next.js OG image convention](https://nextjs.org/docs/app/api-reference/file-conventions/metadata/opengraph-image)).
- Use 1200x630, readable short title, Term/UI mark, restrained terminal motif, and descriptive alt. Test long titles, apostrophes, Unicode, font loading, and unknown slugs.
- If an article has approved hero art, use it consistently in the page, metadata, and JSON-LD. Otherwise use generated OG art; do not accidentally let the global fallback override it.

## 9. Implementation phases and acceptance criteria

### Phase 0 — Validate editorial targets

- [ ] Review Search Console queries/pages for Ratatui, Rust TUI, terminal UI, and current widget docs.
- [ ] Check live SERPs and query phrasing for all 12 briefs; record one intent and one primary page per cluster.
- [ ] Check current docs/routes for overlap; adjust or drop briefs that would duplicate an existing page.
- [ ] Finish query validation and source research for all 12 before implementation reaches release readiness.
- [ ] If a brief fails validation, replace it with a distinct, evidence-supported brief while preserving the 12-piece launch cohort and four categories.

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
- [ ] Add article hub, all 12 static slug routes, all four category routes, author index/profile routes, and 404 behavior.
- [ ] Render initial hub content and category/article links in server HTML; implement filter/search enhancement with invalid query handling and `noindex,follow` metadata.
- [ ] Reuse shadcn and current Term/UI components. Match existing light/dark theme and responsive behavior.
- [ ] Add the article hub to a visible site entry point and add contextual links from relevant docs pages.

**Done when:** direct URL loads, navigation, browser refresh, narrow view, keyboard use, unknown slug/category, and static production rendering behave correctly.

### Phase 3 — Article reading experience

- [ ] Add byline/profile link, accurate date labels, reading time, category breadcrumbs, H2 TOC, share links, code copy, and responsive MDX typography.
- [ ] Add relevant bottom-of-article links/CTA to Term/UI docs, widgets, installation, or GitHub.
- [ ] Add `Max Wells` profile with the approved avatar and verified Term/UI-specific role/bio/social URLs.
- [ ] Add `/authors` with meaningful author information and article counts; no fabricated team roster.

**Done when:** articles are readable, navigable, copyable, and clearly authored on desktop/mobile and keyboard-only.

### Phase 4 — SEO, discovery, and social cards

- [ ] Generate unique metadata and self-canonicals for hub, category, article, and author routes.
- [ ] Add Article/BlogPosting, BreadcrumbList, CollectionPage/ItemList, and Person JSON-LD where content supports them.
- [ ] Add hub, category, author, and per-article generated OG images plus optional real hero-image support.
- [ ] Extend sitemap with all 12 articles, all four categories, the hub, and valid author routes using real publication/update dates.
- [ ] Audit `robots.txt`, canonical origin, image URLs, metadata on production domain, and internal links.

**Done when:** no duplicate canonical targets, missing metadata, broken assets, invalid JSON-LD, or query-param sitemap URLs remain.

### Phase 5 — Prepare the single launch cohort

- [ ] Draft and technically review all 12 articles before the production launch.
- [ ] Compile/run every code example against repository-compatible Ratatui versions.
- [ ] Complete all four category introductions, author pages, thumbnails/OG assets, metadata, internal links, and sitemap entries before release.
- [ ] Set each `publish_date` to the actual shared release date; never pre-publish part of the cohort or backdate articles.
- [ ] Stage every article, category, author page, navigation link, and sitemap change for one coordinated production deployment.
- [ ] Revisit `last_updated` only when code/API, upstream facts, or substantial editorial guidance changes.

**Done when:** all 12 articles and every supporting index/category/author page pass review and can ship in the same release; none depends on a later content batch.

### Phase 6 — Release and ongoing measurement

- [ ] Run repository `pnpm check:quality` and production `pnpm build:production`; verify articles do not require Rust demo generation/toolchain unless an article asset actually does.
- [ ] Inspect generated HTML for title, description, canonical, visible text, links, OG/Twitter tags, and JSON-LD.
- [ ] Verify all 12 article URLs, all four category URLs, the hub, author pages, sitemap, and robots response on the production deployment.
- [ ] Confirm every intended editorial URL returns 200, has a self-canonical, is not blocked or `noindex`, appears in crawlable HTML links and the sitemap, and has valid metadata/schema.
- [ ] Submit/update the sitemap in Search Console on launch day; monitor indexing, impressions, clicks, CTR, and query/page match. Google controls actual index inclusion; submission cannot guarantee it.
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
- [ ] Hub, all 12 detail pages, all four category pages, and author pages are statically generated and return correct 404s.
- [ ] Existing site header/footer appear exactly once on every article/author route.
- [ ] Keyboard navigation, reduced viewport widths, code copy, and image alt behavior work.

### Search and sharing

- [ ] Each public route has a unique title/description and correct self-canonical on `termui.rustify.app`.
- [ ] Filter query views cannot become duplicate indexable pages.
- [ ] Sitemap lists all canonical launch articles, all four category pages, the hub, and valid author pages with honest dates.
- [ ] JSON-LD validates and exactly reflects visible author/date/title/breadcrumb data.
- [ ] OG image previews are readable and correct for the hub, category and author pages, and representative long/short article titles.
- [ ] No `meta keywords`, blocked editorial routes, empty category pages, fake freshness, thin query variants, or fabricated author expertise.

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
