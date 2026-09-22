import type * as React from "react";

/**
 * Rustify's hero / section block: a mono `[ KICKER ]` eyebrow (the square
 * brackets in the brand `primary` colour, like Rustify's accent `[ BLOG ]`)
 * stacked over a display heading.
 *
 * Presentational, no hooks — server component.
 *
 *   as     — heading tag + size ramp: "h1" (hero) | "h2" (section) | "h3"
 *   align  — "left" (default, sections) | "center" (hero)
 *   muted  — dims the kicker label to `rgba(15,15,16,0.73)`
 *
 * Anything after the heading (a hero subcopy `<p>`, say) is a sibling of this
 * block, not a child — keep it in the caller.
 */

type Level = "h1" | "h2" | "h3";

const HEADING_CLASS: Record<Level, string> = {
  h1: "text-[clamp(36px,6vw,52px)] font-semibold leading-[1.05] tracking-[-0.042em] text-[#0f0f10]",
  h2: "text-[clamp(34px,5vw,52px)] font-semibold leading-[1.05] tracking-[-0.042em] text-[#0f0f10]",
  h3: "text-[clamp(22px,3vw,28px)] font-semibold leading-[1.1] tracking-[-0.03em] text-[#0f0f10]",
};

export function BracketHeading({
  kicker,
  children,
  as = "h2",
  align = "left",
  muted = false,
  className = "",
}: {
  /** bare word for the `[ … ]` eyebrow, e.g. "Articles" */
  kicker: React.ReactNode;
  /** the heading text */
  children: React.ReactNode;
  as?: Level;
  align?: "left" | "center";
  muted?: boolean;
  className?: string;
}) {
  const Tag = as;
  return (
    <div
      className={`flex flex-col gap-3 ${
        align === "center" ? "items-center text-center" : ""
      } ${className}`}
    >
      <span
        className={`font-mono text-[14px] uppercase tracking-[-0.01em] ${
          muted ? "text-[rgba(15,15,16,0.73)]" : "text-[#0f0f10]"
        }`}
      >
        <span className="text-primary">[</span> {kicker}{" "}
        <span className="text-primary">]</span>
      </span>
      <Tag className={HEADING_CLASS[as]}>{children}</Tag>
    </div>
  );
}
