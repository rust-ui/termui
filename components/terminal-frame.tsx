import { cn } from "@/lib/utils";

interface TerminalFrameProps {
  title: string;
  src: string;
  height?: number;
  className?: string;
}

/**
 * macOS-style window chrome around a live Ratatui-over-WASM demo.
 * The chrome is plain CSS; the body is a real Rust binary (ratzilla)
 * running in an iframe, not a replayed recording.
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
      <iframe
        src={src}
        title={title}
        style={{ height }}
        className="w-full border-0 bg-background"
      />
    </div>
  );
}
