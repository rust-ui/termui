interface RatzillaFrameProps {
  title: string;
  src: string;
  height?: number;
}

/** Window chrome around an interactive Ratatui app running through Ratzilla. */
export function RatzillaFrame({
  title,
  src,
  height = 320,
}: RatzillaFrameProps) {
  return (
    <div className="my-6 overflow-hidden rounded-xl border border-border bg-card">
      <div className="flex items-center gap-2 border-b border-border bg-muted px-4 py-2.5">
        <span className="size-3 rounded-full bg-red-500/80" />
        <span className="size-3 rounded-full bg-yellow-500/80" />
        <span className="size-3 rounded-full bg-green-500/80" />
        <span className="ml-2 text-xs text-muted-foreground">{title}</span>
      </div>
      <iframe
        src={`/demos/interactive.html?demo=${encodeURIComponent(src)}`}
        title={title}
        loading="lazy"
        className="block w-full border-0"
        style={{ height }}
      />
    </div>
  );
}
