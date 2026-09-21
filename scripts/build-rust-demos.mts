import { spawnSync } from "node:child_process";
import { promises as fs } from "node:fs";
import path from "node:path";

const root = process.cwd();
const examplesRoot = path.join(root, "examples");
const docsRoot = path.join(root, "content", "docs");
const ratatuiConstants = path.join(root, "constants", "ratatui.ts");
const outputFile = path.join(
  root,
  "lib",
  "termui-registry",
  "previews.generated.json"
);

const bases = await fs.readdir(examplesRoot, { withFileTypes: true });
const names: string[] = [];

for (const base of bases.filter((entry) => entry.isDirectory())) {
  const files = await fs.readdir(path.join(examplesRoot, base.name), {
    withFileTypes: true,
  });
  for (const file of files) {
    if (file.isFile() && file.name.endsWith(".tsx")) {
      names.push(`${base.name}/${file.name.slice(0, -4)}`);
    }
  }
}

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
    })
  );
  return nested.flat();
}

for (const file of await collectMdxFiles(docsRoot)) {
  const source = await fs.readFile(file, "utf8");
  for (const [, tag, attributes] of source.matchAll(
    /<(RustDemo|TerminalFrame)\b([\s\S]*?)\/>/g
  )) {
    const props = new Map(
      [...attributes.matchAll(/\b(name|src)\s*=\s*["']([^"']+)["']/g)].map(
        ([, key, value]) => [key, value]
      )
    );
    if (tag === "RustDemo" && props.has("src")) continue;
    const demoName = tag === "RustDemo" ? props.get("name") : props.get("src");
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
  { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024 }
);

if (result.status !== 0) {
  process.stderr.write(result.stderr);
  throw new Error("Rust demo renderer failed.");
}

await fs.mkdir(path.dirname(outputFile), { recursive: true });
const previews = JSON.parse(result.stdout) as Record<string, string[]>;
await fs.writeFile(outputFile, `${JSON.stringify(previews, null, 2)}\n`);
console.log(`Generated ${Object.keys(previews).length} Rust previews at ${outputFile}`);
