# Term/UI

Term/UI provides copyable Ratatui widgets for Rust terminal applications. Browse the [Ratatui widget catalog](https://termui.rustify.app/docs/widgets) for Rust source and terminal previews, or follow the [installation guide](https://termui.rustify.app/docs/installation) to copy widgets into your app. Widgets are source modules, not a published Cargo crate.

Static Ratatui preview frames come from a small native Rust renderer; stateful browser demos run Ratatui through Ratzilla and WebAssembly.

`termui-renderer` draws Ratatui frames through `TestBackend`; `termui-registry/src/demos` holds one `demo_*.rs` file per demo, including interactive demos, and generates `lib/termui-registry/previews.generated.json`. The docs-driven build fails if a static Rust preview has no registered renderer. Checked-in WebAssembly assets live under `public/demos/`, so production builds do not need a Rust toolchain.

## Structure

- `crates/termui-registry/src/widgets` — copyable Rust widgets.
- `crates/termui-renderer` — Ratatui buffer-to-ANSI renderer.
- `crates/termui-registry` — one source file per demo, exact demo ID registry, and frame generator.
- `crates/termui-registry/src/demos` — static and interactive Ratatui demos; stateful demos compile to WebAssembly with Ratzilla.
- `app`, `components`, `content`, `lib`, `public` — Term/UI web app, docs, and generated demo assets.
- `__TMP/termcn` — local termcn source used for UI parity comparisons.

## Development

```sh
pnpm install
pnpm dev
```

The dev hook regenerates Rust previews. To regenerate them manually:

```sh
pnpm demos:build
```

Run the interactive Ratatui demo by itself:

```sh
pnpm demos:ratzilla:serve
```

Build its checked-in production assets after changing the Rust demo:

```sh
pnpm demos:ratzilla:build
```

Production build and verification:

```sh
pnpm typecheck
pnpm build
```

The Sponsor page is intentionally omitted.

## Production deployment

Term/UI runs on the shared Rustify server at `termui.rustify.app`. GitHub Actions builds an amd64 Docker image, pushes it to Docker Hub, and deploys it over SSH. Host Nginx routes that hostname to the app on `127.0.0.1:5103` and manages HTTPS with Certbot.

Set these repository variables in `rust-ui/termui`:

- `SERVER_IP`: `23.88.45.210`
- `DOCKER_USERNAME`: Docker Hub account name
- `DOCKER_REPOSITORY`: Docker Hub repository name, for example `termui`
- `LETSENCRYPT_EMAIL`: email used for the first TLS certificate

Set these repository secrets:

- `DOCKER_TOKEN`: Docker Hub access token with read and write access
- `SSH_PRIVATE_KEY`: private key whose public key can log in as `root` on shared-apps

In Cloudflare, point the `termui` A record for `rustify.app` to `23.88.45.210`. Keep it DNS only while the first deployment requests its TLS certificate. After the certificate is issued, enable the proxy if desired.

Run **Actions → Build and Deploy Term/UI → Run workflow**. First run with `skip_build` off. Later, `skip_build` can redeploy the existing image.
