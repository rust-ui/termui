"use client";

import type { CSSProperties } from "react";

import previews from "@/domains/ratatui/registry/previews.generated.json";
import { RATATUI_DEMO_BASE } from "@/domains/ratatui/config";
import { useTerminalTheme } from "@/domains/terminal-themes/use-terminal-theme";
import { terminalThemeMap } from "@/domains/terminal-themes";
import { getPreviewKey } from "@/domains/ratatui/registry/protocol";

type AnsiStyle = Pick<
  CSSProperties,
  | "color"
  | "backgroundColor"
  | "fontWeight"
  | "fontStyle"
  | "opacity"
  | "textDecorationLine"
  | "visibility"
>;

type AnsiState = AnsiStyle & { reverse?: boolean };

const ANSI_COLORS = [
  "#000000",
  "#800000",
  "#008000",
  "#808000",
  "#000080",
  "#800080",
  "#008080",
  "#c0c0c0",
  "#808080",
  "#ff0000",
  "#00ff00",
  "#ffff00",
  "#0000ff",
  "#ff00ff",
  "#00ffff",
  "#ffffff",
];

function indexedColor(index: number) {
  if (index < ANSI_COLORS.length) return ANSI_COLORS[index];
  if (index >= 232) {
    const value = 8 + (index - 232) * 10;
    return `rgb(${value}, ${value}, ${value})`;
  }
  const levels = [0, 95, 135, 175, 215, 255];
  const value = index - 16;
  return `rgb(${levels[Math.floor(value / 36)]}, ${levels[Math.floor(value / 6) % 6]}, ${levels[value % 6]})`;
}

function basicColor(code: number) {
  const index = code >= 90 ? code - 90 + 8 : code - 30;
  return ANSI_COLORS[index];
}

function snapshotStyle(
  state: AnsiState,
  foreground: string,
  background: string,
): AnsiStyle {
  const { reverse, ...style } = state;
  if (!reverse) return style;
  return {
    ...style,
    color: state.backgroundColor ?? background,
    backgroundColor: state.color ?? foreground,
  };
}

function setDecoration(style: AnsiState, decoration: string, enabled: boolean) {
  const decorations = new Set(
    style.textDecorationLine?.split(" ").filter((value) => value.length > 0) ?? [],
  );
  if (enabled) decorations.add(decoration);
  else decorations.delete(decoration);
  const value = [...decorations].join(" ");
  if (value) style.textDecorationLine = value;
  else delete style.textDecorationLine;
}

function parseAnsiLine(line: string, foreground: string, background: string) {
  const segments: { text: string; style: AnsiStyle }[] = [];
  const ansiEscape = String.fromCharCode(27);
  const ansi = new RegExp(`${ansiEscape}\\[([\\d;]*)m`, "g");
  let style: AnsiState = {};
  let cursor = 0;
  let match: RegExpExecArray | null;

  while (true) {
    match = ansi.exec(line);
    if (!match) break;

    if (match.index > cursor) {
      segments.push({
        text: line.slice(cursor, match.index),
        style: snapshotStyle(style, foreground, background),
      });
    }
    const codes = match[1].split(";").map(Number);
    const next = { ...style };
    for (let index = 0; index < codes.length; index += 1) {
      const code = codes[index];
      if (code === 0) {
        for (const key of Object.keys(next)) {
          delete next[key as keyof AnsiState];
        }
      } else if (code === 1) {
        next.fontWeight = 700;
      } else if (code === 2) {
        next.opacity = 0.55;
      } else if (code === 3) {
        next.fontStyle = "italic";
      } else if (code === 4) {
        setDecoration(next, "underline", true);
      } else if (code === 7) {
        next.reverse = true;
      } else if (code === 8) {
        next.visibility = "hidden";
      } else if (code === 9) {
        setDecoration(next, "line-through", true);
      } else if (code === 22) {
        delete next.fontWeight;
        delete next.opacity;
      } else if (code === 23) {
        delete next.fontStyle;
      } else if (code === 24) {
        setDecoration(next, "underline", false);
      } else if (code === 27) {
        delete next.reverse;
      } else if (code === 28) {
        delete next.visibility;
      } else if (code === 29) {
        setDecoration(next, "line-through", false);
      } else if ((code >= 30 && code <= 37) || (code >= 90 && code <= 97)) {
        next.color = basicColor(code);
      } else if (code === 39) {
        delete next.color;
      } else if ((code >= 40 && code <= 47) || (code >= 100 && code <= 107)) {
        next.backgroundColor = basicColor(code - 10);
      } else if (code === 49) {
        delete next.backgroundColor;
      } else if ((code === 38 || code === 48) && codes[index + 1] === 2) {
        const color = `rgb(${codes[index + 2]}, ${codes[index + 3]}, ${codes[index + 4]})`;
        if (code === 38) next.color = color;
        else next.backgroundColor = color;
        index += 4;
      } else if ((code === 38 || code === 48) && codes[index + 1] === 5) {
        const color = indexedColor(codes[index + 2]);
        if (code === 38) next.color = color;
        else next.backgroundColor = color;
        index += 2;
      }
    }
    style = next;
    cursor = ansi.lastIndex;
  }

  if (cursor < line.length) {
    segments.push({
      text: line.slice(cursor),
      style: snapshotStyle(style, foreground, background),
    });
  }
  return segments.length > 0
    ? segments
    : [{ text: line, style: snapshotStyle(style, foreground, background) }];
}

export const RatatuiDemoPreview = ({
  name,
  rows = 18,
  fontSize = 12,
}: {
  name: string;
  rows?: number;
  fontSize?: number;
}) => {
  const [themeKey] = useTerminalTheme();
  const theme = terminalThemeMap[themeKey];
  const frame = (previews as Record<string, string[]>)[
    getPreviewKey(RATATUI_DEMO_BASE, name)
  ] ?? [`⚠ Preview not generated: ${name}`];

  return (
    <div
      className="bg-card overflow-auto overscroll-contain"
      style={{
        backgroundColor: theme.colors.background,
        color: theme.colors.foreground,
        height: `${rows * 18 + 20}px`,
        padding: 10,
        width: "100%",
        boxSizing: "border-box",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <pre
        className="m-0 min-w-max select-none font-mono text-xs leading-[18px]"
        style={{ color: theme.colors.foreground, textAlign: "center", fontSize }}
      >
        {frame.map((line, row) => (
          <span key={row}>
            {parseAnsiLine(line, theme.colors.foreground, theme.colors.background).map(
              (segment, index) => (
                <span key={index} style={segment.style}>
                  {segment.text}
                </span>
              ),
            )}
            {row < frame.length - 1 ? "\n" : null}
          </span>
        ))}
      </pre>
    </div>
  );
};
