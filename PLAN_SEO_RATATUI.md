# Term/UI SEO Plan: Ratatui and Rust Terminal UI

Date: 2026-09-21
Status: Repository Phase 1 complete; Phase 2 priority pages and Phase 3 pilot complete; Phase 4 README/link work started. Search Console and live production checks remain unavailable from this workspace.

## Goal

Bring developers searching for Ratatui widgets, Rust terminal UI components, and practical TUI building blocks to Term/UI. Make the product clear: copyable Rust components for Ratatui, with source code and terminal previews. Do not imply that the components are a published Cargo crate.

Primary audience: Rust developers already building, or choosing to build, a terminal application with Ratatui. Their need is practical: find a button, input, dialog, chart, or other reusable interface component and see how to use it.

## Audit summary

### Strong foundation

- Canonical product domain is termui.rs. Existing route aliases redirect to current docs routes.
- seo/metadata.ts centralizes canonical URLs, descriptions, Open Graph, and Twitter metadata. Widget and chart pages generate page-specific metadata.
- app/sitemap.ts and app/robots.txt/route.ts exist. Robots allows crawling and names the sitemap.
- Docs are statically rendered. Site has 45 individual widget pages, Rust source, and Rust-generated previews.
- content/docs/widgets/index.mdx already describes the collection as copyable Ratatui widgets.
- English-only content matches project language policy. No language alternates needed.
- llms.txt, Markdown routes, and RSS support discovery, but do not replace searchable HTML pages. Browser MCP and the Ink/OpenTUI shadcn registry were removed because Term/UI now focuses on copyable Ratatui source.

### Highest-value gaps

1. **Homepage did not name Ratatui prominently** (fixed in Phase 1). Its title was inherited from the general brand slogan, and the visible hero said “Beautiful terminal UIs, made simple.” The site description said Rust terminal UI but omitted Ratatui.
2. **Global structured data did not consistently describe the visible product** (fixed in Phase 1). FAQJsonLd was emitted on every route, but its questions were not visible there. SoftwareSourceCodeJsonLd described the web stack (TypeScript, React, Next.js) instead of the Rust/Ratatui component source, and set dateModified to the render date.
3. **Sitemap timestamps were not trustworthy** (fixed in Phase 1). app/sitemap.ts set every page’s lastModified to new Date(), making unchanged pages appear modified whenever the sitemap was generated. No reliable source dates exist, so the field is now omitted.
4. **Widget pages have useful demos and code, but many have little explanatory prose.** Many widget MDX files have short descriptions and roughly 40–70 words of prose alongside demos and source. Word count alone is not a ranking issue; the opportunity is to answer practical search intent more fully and distinctly.
5. **Search performance baseline is unknown.** No Search Console or analytics setup was found in tracked source. This does not prove external properties are absent. Production crawlability, indexing, selected canonicals, and query data were not confirmed in this audit.
6. **The former keyword metadata mixed adjacent products** (fixed in Phase 1). constants/site.ts included React, Next.js, Ink, and shadcn terms alongside Rust terminal UI. The field was removed; visible page content remains the SEO strategy.
7. **Machine-readable routes need an indexation check.** llms.txt, Markdown, RSS, agent skills, and the API catalog remain. Check Search Console for their indexation and crawl impact; keep only routes that help developers discover or use the Ratatui source.

## Search intent and page map

Treat these as keyword hypotheses, not verified search-volume estimates. Use Search Console and live result pages to refine them before expanding content. Keep one primary intent per URL.

| Search intent | Example searches | Best destination | Page promise |
| --- | --- | --- | --- |
| Learn what Term/UI is | Term/UI, Term/UI Ratatui components | / | Brand overview, accurate workflow, and a direct route into the widget catalog |
| Find a Ratatui component collection | Ratatui components, Ratatui widgets, Ratatui UI components, Rust TUI components | /docs/widgets | Copyable Rust components, source, previews, and clear setup |
| Find a particular widget | Ratatui button widget, Ratatui text input, Ratatui modal/dialog, Ratatui command palette, Ratatui chart | Widget detail pages under /docs/widgets/ | What it does, when to use it, API/source, and a working example |
| Add components to an app | copy Ratatui widgets, reusable Ratatui components, Rust terminal UI component library | /docs/installation | Exact copy workflow, dependencies, and integration boundaries |
| Build a composed interface | Ratatui form, Ratatui dashboard, Ratatui dialog with buttons, Ratatui searchable list | A focused guide only when validated by query data | A real, runnable composition using several Term/UI widgets |
| Explore charts | Ratatui chart examples, Rust terminal chart, Ratatui sparkline | /charts/type | Chart types, data shape, and usable source; link to the Chart source page |

Use both “widget” and “component” naturally: Ratatui users may search either way. Prefer “Ratatui” and “Rust terminal UI/TUI” over broad “Rust UI,” which can mean desktop or web UI. Do not repeat keyword lists in page copy.

### Page ownership and cannibalization

Give each route a distinct job:

- Homepage: brand, product promise, and first click. Link to the widget catalog; do not make it a second full catalog page.
- Widget hub: primary non-branded landing page for Ratatui widgets/components.
- Widget detail pages: one component and its actual API/use case.
- Chart family pages: visual example gallery; the docs Chart page explains source/API and links to the gallery.
- Installation: copy workflow, dependencies, and integration limits.

Review these naturally overlapping pages against Search Console query/page data: Chart vs chart galleries; Table vs Sortable Table; List vs Select List; Progress vs Progress Bar. Preserve both pages where their code and purpose differ, but make that difference explicit in title, description, opening copy, and links. Consolidate or redirect only if the pages actually answer the same intent.

Before writing a guide, inspect the current top results for “Ratatui widgets,” “Ratatui components,” “Rust TUI components,” and one specific widget query. Ratatui’s own site already covers fundamentals; identify what searchers still need and what Term/UI can demonstrate with original code. Do not claim search volume or competitor weakness without data.

## Implementation roadmap

### Work completed in this repository

- Added a Ratatui-specific homepage title, description, hero, and direct widget/setup calls to action.
- Aligned shared site description and source-code/organization JSON-LD with the copyable Rust widget product; removed inaccurate site-wide FAQ markup and unrelated web-stack keywords.
- Removed generated sitemap `lastModified` values because source modification dates are not available.
- Added Ratatui-aware metadata to docs and chart pages; improved the widget hub, introduction, installation workflow, and six priority widget pages with distinct use cases and contextual links.
- Published one copy-ready Ratatui form article with contextual links, using the pilot approach because Search Console data is unavailable.
- Updated the GitHub README to link directly to the widget catalog and installation guide, and clarified that widgets are copyable Rust modules rather than a published crate.
- Removed the unused Ink/OpenTUI registry, browser MCP install flow, registry API surfaces, and TSX demo pipeline; retained Rust widget previews, checked-in Ratzilla assets, and the site theme selector.
- Kept all existing widget URLs and preserved the docs changelog source directory.

Phase 1 repository work is complete. Phase 2 priority pages are improved; remaining component-page expansion waits for query evidence. Phase 3 has one pilot guide. Phase 4 has updated the GitHub README; broader link, redirect, and community outreach work remains. Search Console exports, production URL inspection, live SERP review, and production performance checks still need external access.

### Phase 0 — Establish evidence and indexing baseline

- Verify ownership/access for Google Search Console and Bing Webmaster Tools for https://termui.rs. Add the canonical sitemap in each console if absent.
- Review the live SERP for the priority query groups and record result types, competing component libraries, terminology, and gaps. Use autocomplete/community language as input, then validate with Search Console; do not buy keyword tools or add a new package just for the audit.
- In Search Console, export the last 3–6 months of queries and pages. Group branded vs non-branded queries; record impressions, clicks, CTR, average position, and indexing status for the homepage, widget hub, each widget page, installation, and chart pages.
- Inspect important URLs with URL Inspection: homepage, widget hub, Button, Text Input, Dialog, Charts, and Installation. Confirm crawl allowed, HTTP 200, rendered text, index eligibility, selected canonical, and mobile rendering.
- Check Search Console for llms.txt, Markdown, RSS, agent-skill, and API-catalog URLs. They are supporting developer resources, not intended search landing pages. If indexed or consuming crawl attention, add an appropriate noindex response header without blocking clients that use them.
- Check redirects and production environment values: HTTP/HTTPS and host variants must resolve to the canonical Term/UI domain; production metadata must not contain localhost or a different SITE_URL.
- Record Core Web Vitals/PageSpeed on mobile and desktop, especially widget pages with interactive WebAssembly demos. RustDemo already lazy-loads its interactive iframe; confirm no page eagerly loads unrelated WASM or large assets.
- Confirm sitemap and robots responses in production are 200, text/XML as appropriate, UTF-8, and contain the expected canonical URLs. The current Content-Signal robots line is an extra crawler directive; verify its intended consumers and keep the standard robots directives clear.

**Done when:** baseline export and URL sample are saved; indexing and host/canonical issues have owners and concrete fixes.

### Phase 1 — Fix the highest-impact technical and message issues

- Give the homepage route its own metadata instead of relying on generic root metadata. Set the bare title to **Ratatui UI Components for Rust Terminal Apps**; the existing title template should render **Ratatui UI Components for Rust Terminal Apps | Term/UI**. Proposed description: **Build Rust terminal UIs with copyable Ratatui components. Explore buttons, inputs, dialogs, charts, and more, with source code and terminal previews.**
- Rewrite the visible homepage hero and first paragraph so Ratatui, Rust terminal UI, copyable components, and the product workflow are clear before the component grid. Keep the message accurate: users copy source; components are not a published crate.
- Improve the /docs/widgets hub introduction. Explain who the collection is for, how copying works, how components relate to Ratatui’s built-in widgets, and where to start. Link to high-intent widgets and Installation.
- Align the shared SITE description with the product’s actual Ratatui focus, then keep the homepage’s title/description route-specific so its brand overview does not replace more specific docs metadata. Remove or align off-target keyword metadata only if no other consumer depends on it; do not expect it to improve rankings by itself.
- Remove the site-wide FAQ schema or emit it only on a page where the same FAQ content is visible. Do not add FAQ markup just to chase rich results.
- Correct or simplify JSON-LD in seo/json-ld.tsx:
  - Keep site/organization data only when it accurately describes Term/UI.
  - Use the actual brand logo asset rather than the social preview image as the organization logo.
  - Make software/source-code language data describe the Rust components accurately; keep web implementation technologies separate if mentioned.
  - Remove dateModified unless it comes from a real source update.
  - Keep BreadcrumbList on useful nested docs/chart pages; remove the one-item Home breadcrumb and add missing crumbs only where visible navigation supports that hierarchy.
- Fix sitemap freshness in app/sitemap.ts: emit lastModified only from a reliable content date. If no reliable date exists, omit it. Keep only canonical, indexable HTML URLs; keep noindex launch-week pages and utility endpoints out.
- Verify important routes have unique titles and descriptions, consistent canonical URLs, and appropriate index/noindex status. Keep the existing metadata helper; avoid a new SEO abstraction unless a real gap remains.
- Verify markdown alternates resolve to their matching HTML canonical, and retained API/LLM resources stay usable without becoming unnecessary search results.

**Done when:** homepage source, page copy, and metadata identify the Ratatui component product; structured data matches visible content; sitemap dates and URLs are defensible.

### Phase 2 — Improve component page relevance and usefulness

Keep the existing 45 widget URLs. Do not create near-duplicate pages for keyword variations. Give each page a short, distinct answer to its own use case.

For each page in content/docs/widgets/:

- Use a clear title such as **Ratatui Button Widget** or **Ratatui Text Input Widget**, with the existing Term/UI title template adding the brand once.
- Write a unique description that says what the widget does and names Ratatui/Rust where natural.
- Open with a direct explanation of the component and the problem it solves.
- Add a compact “When to use it” or “What it provides” section grounded in the actual API.
- Explain who owns state and input handling, where applicable. Many widgets render UI but do not implement a full event loop.
- Include a minimal, valid Rust usage example with required imports and surrounding context. Keep existing demos and source viewer; do not replace examples with generic prose.
- Link in context to related widgets and setup. Examples: Text Input ↔ Form Field ↔ Select List; Dialog ↔ Button; Table ↔ Sortable Table; Chart ↔ Sparkline.
- State dependencies and compatibility only when verified against the code and supported Ratatui version.
- Keep titles concise and distinct. Do not append Ratatui to every heading if it makes the title awkward; put the topic where it reads naturally in title, H1, opening copy, and links.

Start with Button, Text Input, Dialog, Table, Chart, and Command Palette. Then use GSC impressions and clicks to choose the next group. Preserve Rust-generated static frames as the default; keep WebAssembly for demos whose state changes meaningfully.

**Done when:** every priority component page has unique metadata, an accurate opening answer, valid source usage, and contextual links without repetitive boilerplate.

### Phase 3 — Build a small set of high-intent guides

Use Search Console data to choose topics. If data is not available yet, publish one pilot guide and assess before scaling. Candidate topics:

1. **How to compose reusable widgets in Ratatui** — explain what Ratatui’s Widget trait provides and where copyable higher-level components help.
2. **Build a Ratatui form** — compose Text Input, Text Area, Select List, Checkbox, and Form Field with parent-owned state and validation.
3. **Build a Ratatui dashboard** — combine Chart, Sparkline, Table, Status Bar, and layout components using realistic data.
4. **Ratatui dialogs and confirmation prompts** — show event handling, state ownership, focus, and dialog composition.

Each guide must include original, working code; explain tradeoffs; link to relevant component pages; and answer a distinct user task. Ratatui’s tutorials and widget reference already cover fundamentals, so Term/UI guides should focus on useful composition and copy-ready UI patterns rather than rewrite those docs.

Do not set a word-count quota. Do not make pages by swapping widget names into the same template. Do not publish year-stamped “latest” content unless the topic genuinely changes with a release.

**Done when:** each new guide has a distinct query intent, a tested example, internal links to the component cluster, and Search Console review after indexing.

### Phase 4 — Strengthen internal and external discovery

- Ensure the homepage links to /docs/widgets, Installation, and the most useful widget pages with descriptive anchor text.
- Ensure the widget hub links to all components with descriptive labels and organizes them by task where that helps users.
- Add contextual links among related pages, not just sidebar or previous/next navigation. Review route redirects so internal links use canonical destinations directly.
- Audit all docs links, MDX links, and navigation destinations for dead routes, redirect chains, and links that use only vague labels such as “here.”
- Keep the project GitHub README linked to the widget hub and installation guide; remove or update any remaining registry-facing instructions.
- Share real component examples in relevant Ratatui/Rust communities and submit the project to maintained ecosystem directories when appropriate. Contribute useful examples upstream. Do not buy links or use mass directory submissions.
- Keep the sitemap, RSS, and Markdown/LLM endpoints aligned with canonical public content. Do not add machine-readable endpoints to the HTML sitemap unless they are intended as search landing pages.

**Done when:** every target page is reachable through crawlable links, repo/community links point to the best matching landing page, and no internal links target legacy routes or dead URLs.

### Phase 5 — Maintain and iterate

- Review Search Console monthly by query and page. Find pages with relevant impressions but weak CTR; improve title/description and visible opening only when they do not match the query.
- Watch indexing reports for “crawled, currently not indexed,” duplicate URLs, and Google-selected canonicals. Check page quality and intent before requesting reindexing.
- Compare non-branded clicks to Ratatui/Rust TUI clusters before and after each release. Keep a change log of URL, query group, edit, date, and result.
- Recheck links, metadata, sitemap URLs, structured data, mobile performance, and important demos after major releases.
- Add a lightweight metadata/SEO check to existing CI only if manual audits keep finding regressions. Check page metadata coverage, duplicate/empty titles, canonical host, sitemap/noindex alignment, and truthful structured data. Do not add a package for this.

## Measurement

Primary measures:

- Non-branded Search Console clicks and impressions for Ratatui/Rust terminal UI query groups.
- Clicks and CTR by target URL, segmented by query group and device.
- Indexed status and canonical selection for the homepage, widget hub, priority widget pages, installation, and charts.

Supporting measures:

- Clicks from organic landing pages to component source, GitHub, and setup instructions, if a privacy-conscious existing analytics setup is available.
- Mobile Core Web Vitals and page load behavior on pages with interactive demos.
- Referring domains and relevant community mentions, reviewed for quality rather than raw count.

Capture a baseline before edits. Compare equal 28-day windows after changes, while accounting for seasonality and search demand. Do not promise a ranking position or treat impressions alone as success.

## Release acceptance checklist

- [ ] Homepage and widget hub have separate search intents and clear Ratatui wording.
- [ ] Priority HTML pages have unique title, description, H1, canonical, and useful opening content.
- [ ] Search-focused copy is visible in initial rendered HTML; interactive demos enhance rather than gate the page’s answer.
- [ ] Every target URL is linked from a crawlable HTML page; legacy routes redirect to the canonical destination.
- [ ] Sitemap includes only canonical, indexable HTML destinations and never fabricates last-modified dates.
- [ ] Robots rules allow search crawling of public pages and advertise the sitemap; machine endpoints remain usable.
- [ ] Structured data matches visible page content, uses truthful dates and language, and passes Google’s Rich Results Test where a supported rich-result type applies.
- [ ] OG/Twitter previews resolve on the canonical domain and use appropriate images and alt text.
- [ ] No duplicate titles/descriptions or unexplained Google-selected canonicals among priority pages.
- [ ] Search Console baseline and follow-up window are saved with changes and query groups annotated.
- [ ] Mobile layout and interaction remain usable; key pages have no avoidable loading or layout-shift regressions.

## RUSTIFY practices to reuse

The RUSTIFY audit reviewed its page-metadata helper, metadata coverage check, SEO changelog, content audit checklist, and strategy notes.

Reuse:

- Page-level titles, descriptions, canonicals, and social metadata. Term/UI already has a good shared helper.
- A simple guard against missing metadata and duplicate brand suffixes.
- Full internal-link audits and redirecting/consolidating duplicate intent.
- Search Console evidence for choosing titles and topics. RUSTIFY’s title CTR fix and cannibalization cleanup are stronger operating patterns than generic keyword guessing.
- Tracking real content refreshes; never change dates only to look fresh.

Do not copy article-only heuristics such as 2,300-word minimums, fixed FAQ counts, or forced comparison tables into widget reference pages. Term/UI pages should be as long as needed to solve the component task and prove the code works.

## Audit limits

- Search Console, Bing Webmaster Tools, backlink reports, and production analytics were not available in this workspace. Their external configuration remains unknown.
- Direct production page retrieval was unavailable during this audit. Live HTTP status, rendered metadata, redirect behavior, index coverage, and current Core Web Vitals must be checked in Phase 0.
- Search phrases above are hypotheses based on product fit and the current Ratatui ecosystem, not paid keyword-volume research.

## References

- [Google Search Essentials](https://developers.google.com/search/docs/essentials)
- [Google SEO Starter Guide](https://developers.google.com/search/docs/fundamentals/seo-starter-guide)
- [Google structured data guidelines](https://developers.google.com/search/docs/appearance/structured-data/sd-policies)
- [Google sitemap guidance](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap)
- [Google canonicalization](https://developers.google.com/search/docs/crawling-indexing/canonicalization)
- [Google helpful, reliable, people-first content](https://developers.google.com/search/docs/fundamentals/creating-helpful-content)
- [Ratatui](https://ratatui.rs/)
- [Ratatui tutorials](https://ratatui.rs/tutorials/)
- [Ratatui widget concepts](https://ratatui.rs/recipes/widgets/)
- [Next.js metadata and OG images](https://nextjs.org/docs/app/getting-started/metadata-and-og-images)
