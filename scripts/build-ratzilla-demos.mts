import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { promises as fs } from "node:fs";
import path from "node:path";
import {
  collectInteractiveDemos as collectInteractiveDemosFromMdx,
  renderInteractiveDemo,
} from "./lib/ratzilla-demos.mts";
import type { InteractiveDemo, MdxSource } from "./lib/ratzilla-demos.mts";

const root = process.cwd();
const docsRoot = path.join(root, "content", "docs");
const packageRoot = path.join(root, "crates", "termui-registry");
const shellRoot = path.join(root, "scripts", "ratzilla-demo-shell");
const publicDemosRoot = path.join(root, "public", "demos");
const templatePath = path.join(
  packageRoot,
  "interactive-demo.template.html"
);
const generatedFiles = new Map<string, string>();

interface DemoAssets {
  title: string;
  hash: string;
  jsIntegrity: string;
  wasmIntegrity: string;
}

async function collectMdxFiles(directory: string): Promise<string[]> {
  const entries = await fs.readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) return collectMdxFiles(entryPath);
      return entry.isFile() && entry.name.endsWith(".mdx") ? [entryPath] : [];
    })
  );
  return nested.flat();
}

async function collectInteractiveDemos(): Promise<InteractiveDemo[]> {
  const sources: MdxSource[] = await Promise.all(
    (await collectMdxFiles(docsRoot)).map(async (file) => ({
      file,
      source: await fs.readFile(file, "utf8"),
    }))
  );
  const demos = collectInteractiveDemosFromMdx(sources);
  if (demos.length === 0) {
    throw new Error(`No interactive Rust demos found under ${docsRoot}`);
  }
  return demos;
}

async function generateHtml(demos: readonly InteractiveDemo[]) {
  const template = await fs.readFile(templatePath, "utf8");

  for (const demo of demos) {
    const html = renderInteractiveDemo(template, demo);
    const target = targetFor(demo);
    const targetPath = path.join(packageRoot, target);
    generatedFiles.set(demo.name, targetPath);
    await fs.writeFile(targetPath, html);
  }
}

function runTrunk(args: string[]) {
  const { NO_COLOR: _noColor, ...env } = process.env;
  const result = spawnSync("trunk", args, {
    cwd: packageRoot,
    env,
    stdio: "inherit",
  });

  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`Trunk failed with exit code ${result.status ?? -1}`);
  }
}

function targetFor(demo: InteractiveDemo) {
  return `.ratzilla-${process.pid}-${demo.name}.generated.html`;
}

function getAssetLink(
  html: string,
  demo: InteractiveDemo,
  rel: string,
  extension: ".js" | ".wasm"
) {
  for (const [tag] of html.matchAll(/<link\b[^>]*>/g)) {
    const attributes = Object.fromEntries(
      [...tag.matchAll(/([\w-]+)="([^"]*)"/g)].map(([, key, value]) => [
        key,
        value,
      ])
    );
    if (attributes.rel !== rel || !attributes.href?.endsWith(extension)) {
      continue;
    }

    const prefix = `/demos/${demo.name}/`;
    if (!attributes.href.startsWith(prefix)) {
      throw new Error(`${demo.name} asset escaped its public directory`);
    }
    if (!attributes.integrity?.startsWith("sha384-")) {
      throw new Error(`${demo.name} ${extension} asset is missing SHA-384 integrity`);
    }

    return {
      file: attributes.href.slice(prefix.length),
      integrity: attributes.integrity,
    };
  }

  throw new Error(`Trunk output is missing ${demo.name} ${extension} asset link`);
}

async function readDemoAssets(demo: InteractiveDemo): Promise<DemoAssets> {
  const demoRoot = path.join(publicDemosRoot, demo.name);
  const html = await fs.readFile(path.join(demoRoot, "index.html"), "utf8");
  const js = getAssetLink(html, demo, "modulepreload", ".js");
  const wasm = getAssetLink(html, demo, "preload", ".wasm");
  await Promise.all([
    fs.access(path.join(demoRoot, js.file)),
    fs.access(path.join(demoRoot, wasm.file)),
  ]);
  const jsHash = js.file.match(
    new RegExp(`^${demo.name}-([a-f0-9]+)\\.js$`)
  )?.[1];
  if (!jsHash || wasm.file !== `${demo.name}-${jsHash}_bg.wasm`) {
    throw new Error(`${demo.name} JS and WASM asset names must share one hash`);
  }

  return {
    title: demo.title,
    hash: jsHash,
    jsIntegrity: js.integrity,
    wasmIntegrity: wasm.integrity,
  };
}

async function writeSharedShell(demos: readonly InteractiveDemo[]) {
  const assets = Object.fromEntries(
    await Promise.all(
      demos.map(async (demo) => [demo.name, await readDemoAssets(demo)] as const)
    )
  );
  const manifest = JSON.stringify(assets, null, 2);
  const manifestVersion = createHash("sha256")
    .update(manifest)
    .digest("hex")
    .slice(0, 12);
  const shell = await fs.readFile(path.join(shellRoot, "index.html"), "utf8");
  const loader = await fs.readFile(
    path.join(shellRoot, "interactive-demo.js"),
    "utf8"
  );
  if (!shell.includes("{{MANIFEST_VERSION}}")) {
    throw new Error("Interactive demo shell is missing its manifest version marker");
  }
  const versionedShell = shell.replaceAll("{{MANIFEST_VERSION}}", manifestVersion);

  await fs.writeFile(
    path.join(publicDemosRoot, "interactive.html"),
    versionedShell
  );
  await Promise.all(
    demos.map((demo) =>
      fs.rm(path.join(publicDemosRoot, demo.name, "index.html"), { force: true })
    )
  );
  await fs.writeFile(
    path.join(publicDemosRoot, "interactive-demo.js"),
    loader
  );
  await fs.copyFile(
    path.join(shellRoot, "interactive-demo.css"),
    path.join(publicDemosRoot, "interactive-demo.css")
  );
  await fs.writeFile(
    path.join(publicDemosRoot, "interactive-demo.json"),
    `${manifest}\n`
  );
}

async function withGeneratedHtml(
  demos: readonly InteractiveDemo[],
  action: () => void
) {
  try {
    await generateHtml(demos);
    action();
  } finally {
    await Promise.all(
      [...generatedFiles.values()].map((file) => fs.rm(file, { force: true }))
    );
    generatedFiles.clear();
  }
}

async function build() {
  const demos = await collectInteractiveDemos();
  await withGeneratedHtml(demos, () => {
    for (const demo of demos) {
      runTrunk([
        "build",
        "--release",
        "--config",
        "Trunk.toml",
        "--dist",
        `../../public/demos/${demo.name}`,
        "--public-url",
        `/demos/${demo.name}/`,
        targetFor(demo),
      ]);
    }
  });
  await writeSharedShell(demos);
}

async function serve(name: string | undefined) {
  const demos = await collectInteractiveDemos();
  const demo = name
    ? demos.find((entry) => entry.name === name)
    : demos.find((entry) => entry.name === "button-interactive");
  if (!demo) {
    throw new Error(`Unknown interactive demo: ${name}`);
  }

  await withGeneratedHtml([demo], () => {
    runTrunk([
      "serve",
      "--config",
      "Trunk.toml",
      "--public-url",
      "/",
      "--serve-base",
      "/",
      "--address",
      "127.0.0.1",
      "--port",
      "8081",
      targetFor(demo),
    ]);
  });
}

const command = process.argv[2] ?? "build";

if (command === "build") {
  await build();
} else if (command === "serve") {
  await serve(process.argv[3]);
} else {
  throw new Error(`Unknown command: ${command}`);
}
