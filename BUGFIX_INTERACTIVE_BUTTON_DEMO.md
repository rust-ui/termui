# Interactive Button Demo Bugfix Plan

**Status:** ✅ Done  
**Date:** 2026-09-21

## Bug

The interactive Button demo at `/docs/widgets/button` rendered its first frame, then trapped in WebAssembly with `RuntimeError: unreachable`. Mouse and keyboard controls stopped responding.

## Cause

The iframe viewport was 370 px tall, while `#terminal` stayed fixed at 320 px. Ratzilla's `DomBackend` sized its DOM grid from `#terminal`; Ratatui sized its terminal buffer from the iframe viewport. Their row counts diverged, and the DOM backend indexed beyond its cell buffer during drawing.

## Fix

- Set `#terminal` height to `100%` so its grid follows the iframe viewport.
- Use the default cursor over the whole terminal grid; `pointer` implied the entire surface was a button.
- Regenerate checked-in Trunk assets for production.
- Record the fix in `CHANGELOG.md`.

## Verification

- `pnpm demos:ratzilla:build` succeeds.
- Playwright reports zero console errors on `/docs/widgets/button`.
- Clicking **Increment +1** increments the counter.
- Space increments; R and clicking **Reset** clear the counter.
- `git diff --check` passes.

## Files

- `crates/termui-registry/index.html`
- `public/demos/button-interactive/`
- `CHANGELOG.md`
