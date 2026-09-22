"use client";

import Link from "next/link";
import type { Route } from "next";
import { ArrowLeft, ArrowRight, type LucideIcon } from "lucide-react";

import { BracketHeading } from "@/components/articles/bracket-heading";
import { useRailEdges } from "@/shared/hooks/use-rail-edges";

/**
 * Rustify's "Browse X" horizontal rail — shared by the /articles hub
 * (CategoryCarousel) and the /wip/blog hub (TopicCarousel). Those two stay as
 * thin `"use client"` wrappers that own their icon-by-key map (a server
 * component can't hand a component reference across the boundary) and shape
 * their taxonomy into `HubCarouselItem[]`; everything visual lives here.
 *
 * Scroll: native `overflow-x-auto` (touch swipe + trackpad for free).
 * `snap-proximity` (not `mandatory`) so a fast flick keeps its momentum;
 * `overscroll-x-contain` stops the swipe bleeding into browser back-nav / page
 * bounce; `scroll-pl` matches the full-bleed padding so a snapped card lands
 * flush with the 1200 column. The rail breaks out of the centered column with
 * `left-1/2 w-screen` and clips at the viewport edge — this needs
 * `overflow-x-clip` on an ancestor (set on each hub's <main>) so it can't add a
 * horizontal scrollbar. `useRailEdges` disables the left / right arrow at each
 * scroll extreme, the way Rustify's carousel toggles the native `disabled`
 * attr on its buttons.
 */

export type HubCarouselItem = {
  /** stable list key + query value */
  key: string;
  /** display name, e.g. "Career Switch" */
  label: string;
  /** 2-3 line card body */
  blurb: string;
  /** article / post count for the pill */
  count: number;
  /** deep link to the filtered hub */
  href: string;
  /** lucide icon, resolved by the wrapper */
  Icon: LucideIcon;
};

const ARROW_CLASS =
  "flex size-10 items-center justify-center rounded-full border border-black/[0.12] bg-white text-[#0f0f10] transition-[opacity,background-color] duration-300 hover:bg-black/[0.03] disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-white";

export function HubCarousel({
  kicker,
  title,
  items,
}: {
  /** bare word for the `[ … ]` eyebrow, e.g. "All categories" */
  kicker: string;
  /** section heading, e.g. "Browse Categories" */
  title: string;
  items: HubCarouselItem[];
}) {
  const { ref: railRef, node: rail, atStart, atEnd } = useRailEdges<HTMLDivElement>();

  function nudge(dir: 1 | -1) {
    rail?.scrollBy({ left: dir * 384, behavior: "smooth" });
  }

  return (
    <section className="mt-28">
      <div className="mb-8 flex items-end justify-between gap-4">
        <BracketHeading as="h2" muted kicker={kicker}>
          {title}
        </BracketHeading>
        <div className="hidden gap-2 sm:flex">
          <button
            type="button"
            onClick={() => nudge(-1)}
            disabled={atStart}
            aria-label={`Scroll ${title} left`}
            className={ARROW_CLASS}
          >
            <ArrowLeft className="size-4" aria-hidden />
          </button>
          <button
            type="button"
            onClick={() => nudge(1)}
            disabled={atEnd}
            aria-label={`Scroll ${title} right`}
            className={ARROW_CLASS}
          >
            <ArrowRight className="size-4" aria-hidden />
          </button>
        </div>
      </div>

      <div className="relative left-1/2 w-screen -translate-x-1/2">
        <div
          ref={railRef}
          className="flex snap-x snap-proximity gap-4 overflow-x-auto overscroll-x-contain scroll-pl-[max(1.5rem,calc((100vw-1200px)/2+1.5rem))] px-[max(1.5rem,calc((100vw-1200px)/2+1.5rem))] pb-4 [-webkit-overflow-scrolling:touch] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
        >
          {items.map(({ key, label: cardLabel, blurb, count, href, Icon }) => (
            <Link
              key={key}
              href={href as Route}
              className="flex w-[300px] shrink-0 snap-start flex-col gap-5 rounded-[36px] border border-black/[0.05] bg-white p-6 shadow-md transition-transform duration-200 hover:-translate-y-0.5"
            >
              <span className="flex size-11 items-center justify-center rounded-[14px] border border-primary/30 bg-primary/10 text-primary">
                <Icon className="size-5" aria-hidden />
              </span>
              <div className="flex flex-col gap-2">
                <h3 className="text-[20px] font-semibold tracking-[-0.03em] text-[#0f0f10]">
                  {cardLabel}
                  <span className="ml-1.5 text-[14px] font-normal tabular-nums text-[rgba(15,15,16,0.4)]">
                    {count}
                  </span>
                </h3>
                <p className="text-[15px] leading-[1.45] text-[rgba(15,15,16,0.73)]">
                  {blurb}
                </p>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </section>
  );
}
