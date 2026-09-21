import { ComponentSource } from "@/components/component-source";
import { MacWindow } from "@/components/mac-window";
import type { TerminalPreviewProps } from "@/components/terminal-preview";
import { TerminalPreview } from "@/components/terminal-preview";
import { TerminalTheme } from "@/components/terminal-theme";
import { cn } from "@/lib/utils";

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
