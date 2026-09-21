import { ComponentSource } from "@/domains/docs/components/component-source";
import { MacWindow } from "@/components/shared/mac-window";
import type { TerminalPreviewProps } from "@/domains/ratatui/components/terminal-preview";
import { TerminalPreview } from "@/domains/ratatui/components/terminal-preview";
import { TerminalTheme } from "@/domains/terminal-themes/components/terminal-theme";
import { cn } from "@/shared/lib/utils";

export const ComponentPreview = ({
  name,
  title = "Terminal",
  className,
  hideCode = false,
  rows,
  theme,
}: TerminalPreviewProps & {
  title?: string;
  className?: string;
  hideCode?: boolean;
}) => (
  <>
    <MacWindow
      className={cn("mt-4", className)}
      title={title}
      trailing={<TerminalTheme />}
    >
      <TerminalPreview name={name} rows={rows} theme={theme} />
    </MacWindow>
    {!hideCode && <ComponentSource name={name} />}
  </>
);
