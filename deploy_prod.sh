#!/usr/bin/env bash
set -euo pipefail

SKIP_BUILD=false
SKIP_PREFLIGHT=false

for arg in "$@"; do
  case "$arg" in
    --skip-build) SKIP_BUILD=true ;;
    --skip-preflight) SKIP_PREFLIGHT=true ;;
    *) echo "Unknown option: $arg" >&2; exit 2 ;;
  esac
done

BRANCH=$(git branch --show-current)
if [[ "$BRANCH" != "main" ]]; then
  echo "Deploy requires main branch (current: $BRANCH)." >&2
  exit 1
fi

if [[ "$SKIP_PREFLIGHT" != true ]] && [[ -n "$(git status --porcelain)" ]]; then
  echo "Working tree is not clean. Commit changes first or pass --skip-preflight to deploy the last commit." >&2
  exit 1
fi

echo "Checking pnpm lockfile..."
pnpm install --frozen-lockfile

echo "Typechecking..."
pnpm typecheck

echo "Building production site..."
pnpm build:production

echo "Checking checked-in interactive demo assets..."
test -s public/demos/button-interactive/index.html
wasm_found=false
js_found=false
for asset in public/demos/button-interactive/*_bg.wasm; do
  if [[ -s "$asset" ]]; then wasm_found=true; break; fi
done
for asset in public/demos/button-interactive/*.js; do
  if [[ -s "$asset" ]]; then js_found=true; break; fi
done
if [[ "$wasm_found" != true || "$js_found" != true ]]; then
  echo "Missing checked-in Ratzilla JavaScript or WebAssembly asset." >&2
  echo "After changing the Rust demo, run pnpm demos:ratzilla:build and commit its output." >&2
  exit 1
fi

echo "Pushing main to origin..."
git push origin main

TAG="deploy_prod_$(date +'%Y%m%d_%H%M%S')"
git tag "$TAG"
git push origin "$TAG"

echo "Triggering GitHub Actions deployment..."
gh workflow run deploy.yml --ref main -f "skip_build=$SKIP_BUILD"

echo "Deployment queued. Tag: $TAG"
echo "Track at: $(gh repo view --json url -q .url)/actions"
