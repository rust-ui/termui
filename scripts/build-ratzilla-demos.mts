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
const templatePath = path.join(
  packageRoot,
  "interactive-demo.template.html"
);
const generatedFiles = new Map<string, string>();

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
