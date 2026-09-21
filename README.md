# Term/UI

Term/UI is part of the Rust/UI ecosystem. It provides Rust terminal UI components and keeps the source site's Next.js documentation and component-site UI. Static preview frames come from a small native Rust renderer. The interactive button demo runs Ratatui in the browser through Ratzilla and WebAssembly.

The static Rust renderer generates `lib/rust-renderer/previews.generated.json`. The Ratzilla demo source lives in `crates/termui-button-interactive`; its prebuilt WebAssembly assets live in `public/demos/button-interactive` so production builds do not need a Rust toolchain.

## Structure

- `crates/termui-widgets` — copyable Rust widgets.
- `crates/termui-renderer` — Rust frame generator for site previews.
- `crates/termui-button-interactive` — live Ratatui button demo compiled to WebAssembly with Ratzilla.
- `app`, `components`, `content`, `lib`, `registry`, `public` — Term/UI web app, docs, and registry assets.
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

Term/UI runs on the shared Rustify server. GitHub Actions builds an amd64 Docker image, pushes it to Docker Hub, and deploys it over SSH. Host Nginx routes `rust-ui.com` to the app on `127.0.0.1:5103` and manages HTTPS with Certbot.

Set these repository variables in `rust-ui/termui`:

- `SERVER_IP`: `23.88.45.210`
- `DOCKER_USERNAME`: Docker Hub account name
- `DOCKER_REPOSITORY`: Docker Hub repository name, for example `termui`
- `LETSENCRYPT_EMAIL`: email used for the first TLS certificate

Set these repository secrets:

- `DOCKER_TOKEN`: Docker Hub access token with read and write access
- `SSH_PRIVATE_KEY`: private key whose public key can log in as `root` on shared-apps

In Cloudflare, point the `@` A record for `rust-ui.com` to `23.88.45.210`. Keep it DNS only while the first deployment requests its TLS certificate. After the certificate is issued, enable the proxy if desired.

Run **Actions → Build and Deploy Term/UI → Run workflow**. First run with `skip_build` off. Later, `skip_build` can redeploy the existing image.
