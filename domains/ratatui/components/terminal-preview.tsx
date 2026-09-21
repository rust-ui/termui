"use client";

import { RatatuiDemoPreview } from "@/domains/ratatui/components/ratatui-demo-preview";
import type { RatatuiComponentName } from "@/domains/ratatui/config";
import type { terminalThemeMap } from "@/domains/terminal-themes";

export interface TerminalPreviewProps {
  name: RatatuiComponentName;
  rows?: number;
  theme?: keyof typeof terminalThemeMap;
}

export const TerminalPreview = ({ name, rows = 18 }: TerminalPreviewProps) => (
  <RatatuiDemoPreview name={name} rows={rows} />
);
