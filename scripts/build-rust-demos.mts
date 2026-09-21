import { spawnSync } from "node:child_process";
import { promises as fs } from "node:fs";
import path from "node:path";
import { collectMdxDemoReferences } from "./lib/demo-mdx.mts";

const root = process.cwd();
const docsRoot = path.join(root, "content", "docs");
const ratatuiConstants = path.join(root, "domains", "ratatui", "config.ts");
const outputFile = path.join(
  root,
  "domains",
  "ratatui",
  "registry",
  "previews.generated.json",
);

const names: string[] = [];

const constantsSource = await fs.readFile(ratatuiConstants, "utf8");
const demoBase = constantsSource.match(/RATATUI_DEMO_BASE\s*=\s*["']([^"']+)["']/)?.[1];
if (!demoBase) {
  throw new Error(`Could not read RATATUI_DEMO_BASE from ${ratatuiConstants}`);
}

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

for (const file of await collectMdxFiles(docsRoot)) {
  const source = await fs.readFile(file, "utf8");
  const isWidgetPage =
    file.startsWith(`${path.join(docsRoot, "widgets")}${path.sep}`) &&
    path.basename(file, ".mdx") !== "index";
  if (isWidgetPage) {
    if (/<TerminalFrame\b/.test(source)) {
      throw new Error(
        `Widget pages must use RustDemo with a Code tab instead of TerminalFrame: ${file}`,
      );
    }
    const rustDemos = [...source.matchAll(/<RustDemo\b([\s\S]*?)\/>/g)];
    if (rustDemos.length === 0) {
      throw new Error(`Widget page has no RustDemo with a Code tab: ${file}`);
    }
    for (const [, attributes] of rustDemos) {
      if (!/\bcode\s*=/.test(attributes)) {
        throw new Error(`Widget RustDemo is missing its code prop: ${file}`);
      }
    }
  }
  for (const { tag, name, src } of collectMdxDemoReferences(source)) {
    if (tag === "RustDemo" && src) continue;
    const demoName = tag === "RustDemo" ? name : src;
    if (!demoName) {
      throw new Error(`Missing ${tag === "RustDemo" ? "name" : "src"} in ${file}`);
    }
    names.push(`${demoBase}/${demoName}`);
  }
}

const uniqueNames = [...new Set(names)].sort();
const result = spawnSync(
  "cargo",
  [
    "run",
    "--quiet",
    "--package",
    "termui-registry",
    "--bin",
    "render-demos",
    "--",
    ...uniqueNames,
  ],
  { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024 },
);

if (result.status !== 0) {
  process.stderr.write(result.stderr);
  throw new Error("Rust demo renderer failed.");
}

await fs.mkdir(path.dirname(outputFile), { recursive: true });
const previews = JSON.parse(result.stdout) as Record<string, string[]>;
await fs.writeFile(outputFile, `${JSON.stringify(previews, null, 2)}\n`);
console.log(`Generated ${Object.keys(previews).length} Rust previews at ${outputFile}`);
