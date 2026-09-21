"use client";

import previews from "@/lib/rust-renderer/previews.generated.json";
import { RATATUI_DEMO_BASE } from "@/constants/ratatui";
import type { RatatuiComponentName } from "@/constants/ratatui";
import { useTerminalTheme } from "@/hooks/use-terminal-theme";
import { terminalThemeMap } from "@/lib/terminal-themes";
import { getPreviewKey } from "@/lib/rust-renderer/protocol";

type AnsiStyle = {
  color?: string;
  backgroundColor?: string;
  fontWeight?: number;
  opacity?: number;
  textDecoration?: string;
};

function parseAnsiLine(line: string) {
  const segments: { text: string; style: AnsiStyle }[] = [];
  const ansi = /\u001b\[([\d;]*)m/g;
  let style: AnsiStyle = {};
  let cursor = 0;
  let match: RegExpExecArray | null;

  while ((match = ansi.exec(line))) {
    if (match.index > cursor) {
      segments.push({ text: line.slice(cursor, match.index), style });
    }
    const codes = match[1].split(";").map(Number);
    const next = { ...style };
    for (let index = 0; index < codes.length; index += 1) {
      const code = codes[index];
      if (code === 0) {
        Object.keys(next).forEach((key) => delete next[key as keyof AnsiStyle]);
      } else if (code === 1) {
        next.fontWeight = 700;
      } else if (code === 2) {
        next.opacity = 0.55;
      } else if (code === 4) {
        next.textDecoration = "underline";
      } else if (code === 22) {
        delete next.fontWeight;
        delete next.opacity;
      } else if (code === 24) {
        delete next.textDecoration;
      } else if ((code === 38 || code === 48) && codes[index + 1] === 2) {
        const color = `rgb(${codes[index + 2]}, ${codes[index + 3]}, ${codes[index + 4]})`;
        if (code === 38) next.color = color;
        else next.backgroundColor = color;
        index += 4;
      }
    }
    style = next;
    cursor = ansi.lastIndex;
  }

  if (cursor < line.length) {
    segments.push({ text: line.slice(cursor), style });
  }
  return segments.length > 0 ? segments : [{ text: line, style: {} }];
}

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
        {frame.map((line, row) => (
          <span key={row}>
            {parseAnsiLine(line).map((segment, index) => (
              <span key={index} style={segment.style}>
                {segment.text}
              </span>
            ))}
            {row < frame.length - 1 ? "\n" : null}
          </span>
        ))}
      </pre>
    </div>
  );
};
