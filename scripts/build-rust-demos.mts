import { spawnSync } from "node:child_process";
import { promises as fs } from "node:fs";
import path from "node:path";

const root = process.cwd();
const examplesRoot = path.join(root, "examples");
const outputFile = path.join(
  root,
  "lib",
  "rust-renderer",
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

names.sort();
names.push(
  "rust/button",
  "rust/button-sizes",
  "rust/button-variants",
  "rust/key-bar",
  "rust/panel",
  "rust/select-list"
);
names.sort();
const result = spawnSync(
  "cargo",
  ["run", "--quiet", "--package", "termui-renderer", "--bin", "render-demos", "--", ...names],
  { cwd: root, encoding: "utf8", maxBuffer: 8 * 1024 * 1024 }
);

if (result.status !== 0) {
  process.stderr.write(result.stderr);
  throw new Error("Rust demo renderer failed.");
}

await fs.mkdir(path.dirname(outputFile), { recursive: true });
await fs.writeFile(outputFile, `${JSON.stringify(JSON.parse(result.stdout), null, 2)}\n`);
console.log(`Generated ${names.length} Rust previews at ${outputFile}`);
