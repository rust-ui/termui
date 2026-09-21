"use client";

import { RustPreview } from "@/components/rust-preview";
import type { terminalThemeMap } from "@/lib/terminal-themes";
import type { BaseName } from "@/registry/bases";
import { DEFAULT_BASE_NAME } from "@/registry/bases";

export interface TerminalPreviewProps {
  base: BaseName;
  name: string;
  rows?: number;
  theme?: keyof typeof terminalThemeMap;
}

export const TerminalPreview = ({
  base = DEFAULT_BASE_NAME,
  name,
  rows = 18,
}: TerminalPreviewProps) => (
  <RustPreview base={base} name={name} rows={rows} />
);
