import fs from "node:fs";
import path from "node:path";
import { TerminalFrame } from "@/components/terminal-frame";

const SOURCE_PATH = path.join(
  process.cwd(),
  "../rust/crates/termui-widgets/src/select_list.rs"
);

export default function SelectListPage() {
  const source = fs.readFileSync(SOURCE_PATH, "utf8");

  return (
    <main className="mx-auto max-w-3xl px-6 py-16">
      <h1 className="text-3xl font-semibold">Select List</h1>
      <p className="mt-2 text-muted-foreground">
        Selectable rows with a full-width highlight on the active row. Try it
        below with the arrow keys.
      </p>

      <TerminalFrame
        title="select_list.rs"
        src="/demos/select-list/index.html"
        height={220}
        className="mt-8"
      />

      <h2 className="mt-10 text-lg font-medium">Installation</h2>
      <pre className="mt-3 overflow-x-auto rounded-lg border border-border bg-card p-4 text-sm">
        <code>cargo add ratatui</code>
      </pre>

      <h2 className="mt-8 text-lg font-medium">Source</h2>
      <pre className="mt-3 overflow-x-auto rounded-lg border border-border bg-card p-4 text-sm">
        <code>{source}</code>
      </pre>
    </main>
  );
}
