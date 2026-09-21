"use client";

import { RustPreview } from "@/components/rust-preview";
import { RATATUI_DEMO_BASE } from "@/constants/ratatui";
import type { RatatuiComponentName } from "@/constants/ratatui";
import type { terminalThemeMap } from "@/lib/terminal-themes";

export interface TerminalPreviewProps {
  name: RatatuiComponentName;
  rows?: number;
  theme?: keyof typeof terminalThemeMap;
}

export const TerminalPreview = ({
  name,
  rows = 18,
}: TerminalPreviewProps) => (
  <RustPreview name={name} rows={rows} />
);
