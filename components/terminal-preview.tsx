"use client";

import { RatatuiDemoPreview } from "@/components/ratatui-demo-preview";
import type { RatatuiComponentName } from "@/constants/ratatui";
import type { terminalThemeMap } from "@/lib/terminal-themes";

export interface TerminalPreviewProps {
  name: RatatuiComponentName;
  rows?: number;
  theme?: keyof typeof terminalThemeMap;
}

export const TerminalPreview = ({ name, rows = 18 }: TerminalPreviewProps) => (
  <RatatuiDemoPreview name={name} rows={rows} />
);
