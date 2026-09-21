"use client";

import previews from "@/lib/rust-renderer/previews.generated.json";
import { RATATUI_DEMO_BASE } from "@/constants/ratatui";
import type { RatatuiComponentName } from "@/constants/ratatui";
import { useTerminalTheme } from "@/hooks/use-terminal-theme";
import { terminalThemeMap } from "@/lib/terminal-themes";
import { getPreviewKey } from "@/lib/rust-renderer/protocol";

export const RustPreview = ({
  name,
  rows = 18,
}: {
  name: RatatuiComponentName;
  rows?: number;
}) => {
  const [themeKey] = useTerminalTheme();
  const theme = terminalThemeMap[themeKey];
  const frame = (previews as Record<string, string[]>)[
    getPreviewKey(RATATUI_DEMO_BASE, name)
  ] ?? [
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
