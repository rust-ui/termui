"use client";

import previews from "@/lib/rust-renderer/previews.generated.json";
import { useTerminalTheme } from "@/hooks/use-terminal-theme";
import { terminalThemeMap } from "@/lib/terminal-themes";
import { getPreviewKey } from "@/lib/rust-renderer/protocol";

export const RustPreview = ({
  base,
  name,
  rows = 18,
}: {
  base: string;
  name: string;
  rows?: number;
}) => {
  const [themeKey] = useTerminalTheme();
  const theme = terminalThemeMap[themeKey];
  const frame = (previews as Record<string, string[]>)[getPreviewKey(base, name)] ?? [
    "╭─ Rust terminal preview ─────────────────╮",
    `│ ${name.slice(0, 38).padEnd(38)} │`,
    "│ Rust example output                     │",
    "╰────────────────────────────────────────╯",
  ];

  return (
    <div
      className="bg-card overflow-auto overscroll-contain"
      style={{
        backgroundColor: theme.colors.background,
        color: theme.colors.foreground,
        height: `${rows * 18 + 20}px`,
        padding: 10,
      }}
    >
      <pre
        className="m-0 min-w-max font-mono text-xs leading-[18px]"
        style={{ color: theme.colors.foreground }}
      >
        {frame.join("\n")}
      </pre>
    </div>
  );
};
