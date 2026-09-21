import fs from "node:fs";
import path from "node:path";
import { TerminalFrame } from "@/components/terminal-frame";

const SOURCE_PATH = path.join(
  process.cwd(),
  "crates/termui-widgets/src/panel.rs"
);

export default function PanelPage() {
  const source = fs.readFileSync(SOURCE_PATH, "utf8");

  return (
    <main className="mx-auto max-w-3xl px-6 py-16">
      <h1 className="text-3xl font-semibold">Panel</h1>
      <p className="mt-2 text-muted-foreground">
        Rounded panel shell, ready to attach to any Ratatui widget via
        <code className="mx-1 rounded bg-muted px-1.5 py-0.5">.block()</code>.
      </p>

      <TerminalFrame title="panel.rs" src="/demos/panel/index.html" className="mt-8" />

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
