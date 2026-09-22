"use client";

import { cn } from "@/shared/lib/utils";

/**
 * Multi-select filter chips — the Rustify sidebar "CATEGORIES / TOPICS" row.
 * Presentational only: the caller owns the selection Set and toggles it.
 * Pressed state uses the `primary` utilities (Rustify brand orange, `--primary`
 * in src/styles.css). Generic over the key type so a closed enum keeps its
 * literal type end-to-end.
 */

export type ChipItem<T extends string = string> = {
  key: T;
  label: string;
  count?: number;
};

export function Chips<T extends string>({
  items,
  selected,
  onToggle,
  className,
}: {
  items: ChipItem<T>[];
  selected: ReadonlySet<T>;
  onToggle: (key: T) => void;
  className?: string;
}) {
  return (
    <div className={cn("flex flex-wrap gap-1.5", className)}>
      {items.map((c) => {
        const on = selected.has(c.key);
        return (
          <button
            key={c.key}
            type="button"
            onClick={() => onToggle(c.key)}
            aria-pressed={on}
            className={cn(
              "rounded-full border px-3 py-1 text-[12px] font-medium uppercase tracking-[-0.01em] transition-colors",
              on
                ? "border-primary/30 bg-primary/10 text-primary"
                : "border-black/[0.08] bg-black/[0.03] text-[rgba(15,15,16,0.73)] hover:bg-black/[0.06]",
            )}
          >
            {c.label}
            {typeof c.count === "number" ? (
              <span className="ml-1 tabular-nums opacity-50">{c.count}</span>
            ) : null}
          </button>
        );
      })}
    </div>
  );
}
