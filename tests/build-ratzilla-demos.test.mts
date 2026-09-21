import assert from "node:assert/strict";
import { promises as fs } from "node:fs";
import path from "node:path";
import test from "node:test";

import { collectMdxDemoReferences } from "../scripts/lib/demo-mdx.mts";
import {
  collectInteractiveDemos,
  renderInteractiveDemo,
} from "../scripts/lib/ratzilla-demos.mts";

const root = process.cwd();

async function collectMdxFiles(directory: string): Promise<string[]> {
  const entries = await fs.readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) return collectMdxFiles(entryPath);
      return entry.isFile() && entry.name.endsWith(".mdx") ? [entryPath] : [];
    }),
  );
  return nested.flat();
}

test("MDX reader distinguishes static and interactive Rust demos", () => {
  const references = collectMdxDemoReferences(`
    <RustDemo name="button" />
    <RustDemo
      name="dialog"
      src="/demos/interactive.html?demo=dialog-interactive"
    />
    <TerminalFrame title="Panel" src="panel" />
  `);

  assert.deepEqual(references, [
    { tag: "RustDemo", name: "button", src: undefined },
    {
      tag: "RustDemo",
      name: "dialog",
      src: "/demos/interactive.html?demo=dialog-interactive",
    },
    { tag: "TerminalFrame", name: undefined, src: "panel" },
  ]);
});

test("interactive demo discovery deduplicates MDX references and validates paths", () => {
  const demos = collectInteractiveDemos([
    {
      file: "widgets/dialog.mdx",
      source: '<RustDemo src="/demos/interactive.html?demo=dialog-interactive" />',
    },
    {
      file: "widgets/button.mdx",
      source: [
        '<RustDemo src="/demos/interactive.html?demo=button-interactive" />',
        '<RustDemo src="/demos/interactive.html?demo=dialog-interactive" />',
        '<RustDemo name="static-demo" />',
      ].join("\n"),
    },
  ]);

  assert.deepEqual(demos, [
    { name: "button-interactive", title: "Interactive Ratatui Button" },
    { name: "dialog-interactive", title: "Interactive Ratatui Dialog" },
  ]);
  assert.throws(
    () =>
      collectInteractiveDemos([
        {
          file: "widgets/broken.mdx",
          source: '<RustDemo src="/demos/dialog.html" />',
        },
      ]),
    /Unsupported interactive demo src.*widgets\/broken\.mdx/,
  );
});

test("HTML template fills bins and escapes page titles", async () => {
  const template = await fs.readFile(
    path.join(root, "crates/termui-registry/interactive-demo.template.html"),
    "utf8",
  );
  const html = renderInteractiveDemo(template, {
    name: "dialog-interactive",
    title: 'Dialog & "Confirm"',
  });

  assert.match(html, /data-bin="dialog-interactive"/);
  assert.match(html, /<title>Dialog &amp; &quot;Confirm&quot;<\/title>/);
  assert.match(html, /event\.key === "Tab"/);
  assert.match(html, /event\.key === "Backspace"/);
  assert.doesNotMatch(html, /\{\{[A-Z_]+\}\}/);
  assert.throws(
    () =>
      renderInteractiveDemo("<title>{{PAGE_TITLE}}</title>", {
        name: "dialog-interactive",
        title: "Dialog",
      }),
    /Missing template marker \{\{BIN_NAME\}\}/,
  );
});

test("interactive shell keeps viewport bounds and keyboard handling shared", async () => {
  const styles = await fs.readFile(
    path.join(root, "scripts/ratzilla-demo-shell/interactive-demo.css"),
    "utf8",
  );
  const loader = await fs.readFile(
    path.join(root, "scripts/ratzilla-demo-shell/interactive-demo.js"),
    "utf8",
  );
  const terminalStyles = styles.match(/#terminal\s*\{([^}]+)\}/)?.[1];

  assert.ok(terminalStyles, "shared shell must size the Ratzilla terminal");
  assert.match(terminalStyles, /\bwidth:\s*100%;/);
  assert.match(terminalStyles, /\bheight:\s*100%;/);
  assert.match(terminalStyles, /\bpadding:\s*0;/);
  assert.match(loader, /event\.key === "Tab"/);
  assert.match(loader, /event\.key === "Backspace"/);
});

test("MDX demos match checked-in Rust previews and interactive assets", async () => {
  const docsRoot = path.join(root, "content", "docs");
  const mdxFiles = await collectMdxFiles(docsRoot);
  const sources = await Promise.all(
    mdxFiles.map(async (file) => ({
      file,
      source: await fs.readFile(file, "utf8"),
    })),
  );
  const references = sources.flatMap(({ source }) => collectMdxDemoReferences(source));
  const demos = collectInteractiveDemos(sources);
  const interactiveAssets = JSON.parse(
    await fs.readFile(path.join(root, "public/demos/interactive-demo.json"), "utf8"),
  ) as Record<
    string,
    {
      title: string;
      hash: string;
      jsIntegrity: string;
      wasmIntegrity: string;
    }
  >;
  const cargoToml = await fs.readFile(
    path.join(root, "crates/termui-registry/Cargo.toml"),
    "utf8",
  );
  const cargoBins = new Set(
    [...cargoToml.matchAll(/\[\[bin\]\][\s\S]*?\bname\s*=\s*"([^"]+)"/g)].map(
      ([, name]) => name,
    ),
  );

  assert.ok(demos.length > 0, "MDX must reference at least one interactive demo");
  for (const demo of demos) {
    assert.ok(cargoBins.has(demo.name), `${demo.name} must be a Cargo binary`);

    const html = await fs.readFile(
      path.join(root, "public/demos/interactive.html"),
      "utf8",
    );
    const assets = interactiveAssets[demo.name];

    assert.ok(assets, `${demo.name} must have a manifest entry`);
    assert.equal(assets.title, demo.title);
    assert.ok(
      references.some(
        (reference) => reference.src === `/demos/interactive.html?demo=${demo.name}`,
      ),
      `${demo.name} MDX must use the shared demo page`,
    );
    assert.match(html, /\/demos\/interactive-demo\.css/);
    assert.match(html, /\/demos\/interactive-demo\.js\?v=/);
    assert.match(html, /<div id="terminal"><\/div>/);
    assert.match(assets.hash, /^[a-f0-9]+$/);
    assert.match(assets.jsIntegrity, /^sha384-/);
    assert.match(assets.wasmIntegrity, /^sha384-/);
    await fs.access(
      path.join(root, "public/demos", demo.name, `${demo.name}-${assets.hash}.js`),
    );
    await fs.access(
      path.join(root, "public/demos", demo.name, `${demo.name}-${assets.hash}_bg.wasm`),
    );
    await assert.rejects(
      fs.access(path.join(root, "public/demos", demo.name, "index.html")),
      { code: "ENOENT" },
    );
  }

  for (const file of [
    "interactive.html",
    "interactive-demo.js",
    "interactive-demo.css",
  ]) {
    await fs.access(path.join(root, "public/demos", file));
  }

  const constants = await fs.readFile(path.join(root, "constants/ratatui.ts"), "utf8");
  const demoBase = constants.match(/RATATUI_DEMO_BASE\s*=\s*["']([^"']+)["']/)?.[1];
  assert.ok(demoBase, "RATATUI_DEMO_BASE must be defined");

  const previews = JSON.parse(
    await fs.readFile(
      path.join(root, "lib/termui-registry/previews.generated.json"),
      "utf8",
    ),
  ) as Record<string, unknown>;
  const staticRefs = references.flatMap((reference) => {
    if (reference.tag === "RustDemo" && !reference.src) {
      return reference.name ? [reference.name] : [];
    }
    if (reference.tag === "TerminalFrame") {
      return reference.src ? [reference.src] : [];
    }
    return [];
  });

  assert.ok(staticRefs.length > 0, "MDX must reference static demos");
  for (const name of new Set(staticRefs)) {
    const frames = previews[`${demoBase}/${name}`];
    assert.ok(
      Array.isArray(frames) && frames.length > 0,
      `${name} needs generated frames`,
    );
    assert.ok(
      frames.every((frame) => typeof frame === "string"),
      `${name} frames must be text`,
    );
  }
});
