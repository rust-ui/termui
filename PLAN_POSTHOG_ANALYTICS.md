# Term/UI PostHog Analytics Plan

Date: 2026-09-22
Status: Proposed; no analytics vendor is installed by this plan

## Goal

Add privacy-conscious product analytics to Term/UI only when the project needs
behavioral evidence beyond Google Search Console.

Measure whether developers discover, inspect, copy, and try the copyable
Ratatui widgets. Keep Term/UI independent from RUSTIFY's product and funnel
analytics.

Google Search Console remains the source for Google Search impressions, clicks,
queries, indexing, and average position. PostHog cannot replace it.

## Current decision

The current SEO plan intentionally uses Search Console only and has no
analytics vendor. Do not add PostHog as part of routine SEO or content work.
Start this plan only after a concrete product question requires behavioral data,
for example:

- Which widget pages lead to source-copy actions?
- Do interactive demos receive meaningful use?
- Which docs or navigation paths lead developers to GitHub?

## Why PostHog

RUSTIFY already uses PostHog with `posthog-js`, a typed event catalog, a
centralized client wrapper, bot filtering, an engagement gate, route pageviews,
and optional server-side conversion capture. Term/UI can reuse the pattern's
boundaries without copying RUSTIFY's lead, identity, attribution, or funnel
logic.

PostHog is preferred over adding Google Analytics because it gives Term/UI one
explicit event surface and matches the existing RUSTIFY operational knowledge.
No Google Analytics, Google Tag Manager, or third-party advertising tracker is
needed.

## Scope

### In scope

- Anonymous pageview and interaction measurement.
- A small typed event catalog owned by Term/UI.
- One client analytics boundary and one capture wrapper.
- Bot and automation exclusion.
- Engagement-gated collection so passive visits and crawlers do not create
  unnecessary events.
- Production environment configuration for the PostHog project key and host.
- A short privacy notice describing the collected events and retention policy.

### Out of scope

- Search Console query, impression, click, or ranking data.
- User accounts, email identification, session replay, surveys, or feature flags.
- Recording raw source code, search text, form values, URL query values, or
  terminal input.
- Reproducing RUSTIFY's conversion funnel or server-side lead capture.
- Adding analytics to Rust widgets or checked-in WASM demo assets.
- Tracking third-party registry content or external sites.

## Proposed architecture

Keep analytics isolated from UI components and feature code:

```text
components/features
        |
        v
shared/lib/analytics/index.ts   typed public API
        |
        v
components/shared/posthog-provider.tsx   initialization + pageviews
        |
        v
posthog-js                              external transport
```

Suggested files:

- `shared/lib/analytics/catalog.ts`: event names and property types.
- `shared/lib/analytics/client.ts`: `capture`, `captureOnce`, and safe guards.
- `shared/lib/analytics/index.ts`: public exports.
- `components/shared/posthog-provider.tsx`: client initialization and route
  pageviews.
- `components/shared/analytics-boundary.tsx` only if a separate Suspense or
  consent boundary is needed.

Do not import `posthog-js` directly from widgets, pages, Rust demo code, or
individual buttons. Add an architecture guard if direct imports begin to spread.

## Initial event catalog

Use stable slugs and bounded enums. Attach `widget_slug`, `route`, and
`surface` only where useful. Never attach raw code or free-form user input.

| Event | Meaning | Initial properties |
| --- | --- | --- |
| `$pageview` | An engaged page view | PostHog standard URL context only |
| `widget_demo_viewed` | Demo became visible | `widget_slug`, `demo_mode` |
| `widget_demo_interacted` | User interacted with a stateful demo | `widget_slug`, `interaction` |
| `widget_source_copied` | Rust source was copied | `widget_slug`, `source_kind` |
| `demo_code_toggled` | User opened the code side of a demo | `widget_slug`, `demo_mode` |
| `docs_search_used` | Docs search was submitted | `result_count`, `surface` |
| `github_link_clicked` | User opened the repository or source link | `widget_slug`, `surface` |
| `installation_link_clicked` | User opened installation guidance | `surface` |

`demo_mode` should be a bounded value such as `static` or `interactive`.
`interaction` should be a bounded value such as `click`, `keyboard`,
`selection`, or `resize`. Do not send the actual key, typed text, clipboard
contents, or query string.

## Initialization rules

Adapt the useful RUSTIFY safeguards:

- Read `NEXT_PUBLIC_POSTHOG_KEY` and `NEXT_PUBLIC_POSTHOG_HOST` from the
  runtime environment; never hardcode a project key.
- Default to no-op when the key is absent, so local development stays quiet.
- Use `capture_pageview: false` and send the first pageview only after real
  engagement or a documented short dwell threshold.
- Refire `$pageview` for client-side App Router navigation.
- Opt out bots, crawlers, headless browsers, and automated tests using a
  maintained user-agent check.
- Skip analytics inside embedded demos or third-party iframes.
- Set `autocapture: false` unless a specific reviewed event requires it.
- Do not enable session recording, surveys, heatmaps, or performance capture
  by default.

The exact persistence and consent behavior must be chosen before production
release. European visitors must not receive analytics that require consent
before the project's privacy/legal position is settled. If consent is needed,
initialization must wait for consent and denied visitors must remain no-op.

## Environment and deployment

Add names, not secrets, to the project example configuration:

```text
NEXT_PUBLIC_POSTHOG_KEY=
NEXT_PUBLIC_POSTHOG_HOST=https://us.i.posthog.com
```

Set real values only in the production deployment environment. Keep Docker,
Compose, and GitHub Actions environment handling aligned; do not commit keys or
generated PostHog configuration.

Before production use, decide whether the PostHog project should use the US or
EU region, then keep the host consistent across local, preview, and production
configuration.

## Implementation phases

### Phase 0 — Decide and verify

- Confirm the product question that requires analytics.
- Confirm PostHog region, retention, access roles, and privacy/consent policy.
- Create a separate Term/UI PostHog project; never reuse the RUSTIFY project
  key.
- Confirm Search Console remains separately configured for `termui.rs`.
- Define the event catalog and reject raw/free-form properties in review.

**Done when:** owner, purpose, region, retention, consent decision, and event
catalog are written down.

### Phase 1 — Add the minimal client boundary

- Add `posthog-js` using the repository's pinned package-manager workflow.
- Add the provider under the existing root provider tree.
- Add the typed client wrapper and catalog under `shared/lib/analytics/`.
- Implement bot filtering, engagement gating, iframe skipping, and route
  pageviews.
- Add environment names to the appropriate example/deployment documentation.
- Keep absent-key behavior a no-op.

**Done when:** production build works without a key; with a test key, one
engaged pageview and one route change appear in the correct PostHog project.

### Phase 2 — Instrument high-value actions

- Add `widget_demo_viewed` to the shared demo visibility boundary.
- Add `widget_demo_interacted` only to demos whose state changes through user
  input; leave visual-only static previews uninstrumented.
- Add `widget_source_copied` to the existing source-copy action.
- Add GitHub and installation link events at shared link boundaries.
- Add docs search measurement without sending raw query text.

**Done when:** event names and properties match the catalog, and event counts
can answer the original product question.

### Phase 3 — Validate and maintain

- Verify no analytics requests occur for bots, denied consent, missing keys, or
  passive non-engaged visits.
- Test production navigation, static pages, interactive WASM demos, iframes,
  dark mode, and error paths.
- Run `pnpm check:quality` and the relevant browser/component tests.
- Review event volume after 7 and 28 days; remove events that do not answer a
  decision.
- Document every new event and preserve the event catalog as the contract.

**Done when:** analytics is reliable, minimal, privacy-reviewed, and useful for
an explicit product decision.

## Privacy and data rules

- Anonymous by default; no `identify()` for public Term/UI visitors.
- No email, name, IP-derived custom fields, clipboard contents, typed keys,
  search text, source code, or terminal input in event properties.
- Use bounded slugs and enums; reject unbounded values at the TypeScript type
  boundary.
- Do not track every click automatically.
- Keep Search Console and PostHog data operationally separate.
- Add or update the public privacy notice before production collection.
- Provide a documented way to disable analytics in local development and tests.

## Acceptance checklist

- [ ] A concrete product question justifies PostHog.
- [ ] Separate Term/UI PostHog project exists.
- [ ] Region, retention, consent, and privacy notice are decided.
- [ ] `posthog-js` is isolated behind one provider and one typed wrapper.
- [ ] Missing key, bot, iframe, test, and denied-consent paths are no-op.
- [ ] `autocapture`, session recording, surveys, and heatmaps are disabled.
- [ ] No raw user input or source content leaves the browser.
- [ ] Initial events are bounded, documented, and implemented only at shared
  interaction boundaries.
- [ ] Search Console property and canonical sitemap remain independent.
- [ ] Production verification confirms events reach the Term/UI project only.
- [ ] `pnpm check:quality` passes.

## RUSTIFY references

Use these as implementation references, not as files to copy wholesale:

- `RUSTIFY-APP/app_v2_website/src/components/posthog-provider.tsx` — provider,
  bot filtering, engagement gate, route pageviews, and iframe guard.
- `RUSTIFY-APP/app_v2_website/src/lib/analytics/catalog.ts` — typed event
  catalog pattern.
- `RUSTIFY-APP/app_v2_website/src/lib/analytics/client.ts` — centralized
  capture boundary.
- `RUSTIFY-APP/app_v2_website/src/lib/posthog-server.ts` — server capture
  pattern, not needed for the first Term/UI phase.

## Measurement

PostHog success means faster answers to product questions, not maximum event
volume. Review:

- engaged pageviews by route;
- widget demo visibility and interaction rate;
- source-copy rate per widget page;
- GitHub and installation click-through;
- docs search usage and zero-result rate, without storing query text.

Do not use PostHog to claim Google ranking improvement. Use Search Console for
organic search performance and compare equivalent 28-day windows.
