import assert from "node:assert/strict";
import { promises as fs } from "node:fs";
import path from "node:path";
import test from "node:test";

import { RATATUI_DEMO_BASE } from "../constants/ratatui.ts";

const root = process.cwd();

test("Card docs use the checked-in Rust preview", async () => {
  const docs = await fs.readFile(
    path.join(root, "content/docs/widgets/card.mdx"),
    "utf8",
  );
  const previews = JSON.parse(
    await fs.readFile(
      path.join(root, "lib/termui-registry/previews.generated.json"),
      "utf8",
    ),
  ) as Record<string, unknown>;
  const frames = previews[`${RATATUI_DEMO_BASE}/card`];

  assert.match(docs, /<RustDemo\s+name="card"/);
  assert.ok(Array.isArray(frames) && frames.length > 0);

  const ansiEscape = String.fromCharCode(27);
  const ansiPattern = new RegExp(`${ansiEscape}\\[[\\d;]*m`, "g");
  const preview = (frames as string[]).join("\n").replace(ansiPattern, "");
  for (const section of [
    "Card Title",
    "Card Description",
    "Card Content",
    "Card Footer",
  ]) {
    assert.ok(preview.includes(section), `Card preview must contain ${section}`);
  }
});
