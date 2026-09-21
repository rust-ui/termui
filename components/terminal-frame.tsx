import { cn } from "@/lib/utils";
import { RustPreview } from "@/components/rust-preview";
import type { RatatuiComponentName } from "@/constants/ratatui";

interface TerminalFrameProps {
  title: string;
  src: RatatuiComponentName;
  height?: number;
  className?: string;
}

/**
 * macOS-style window chrome around a frame rendered by the native Rust demo renderer.
 */
export function TerminalFrame({
  title,
  src,
  height = 320,
  className,
}: TerminalFrameProps) {
  return (
    <div
      className={cn(
        "overflow-hidden rounded-xl border border-border bg-card",
        className
      )}
    >
      <div className="flex items-center gap-2 border-b border-border bg-muted px-4 py-2.5">
        <span className="h-3 w-3 rounded-full bg-red-500/80" />
        <span className="h-3 w-3 rounded-full bg-yellow-500/80" />
        <span className="h-3 w-3 rounded-full bg-green-500/80" />
        <span className="ml-2 text-xs text-muted-foreground">{title}</span>
      </div>
      <RustPreview name={src} rows={Math.ceil(height / 18)} />
    </div>
  );
}
