"use client";

import { useMemo, useState } from "react";
import Link from "next/link";
import type { Route } from "next";
import { Search } from "lucide-react";

import { Chips } from "@/components/articles/chips";
import { ContentCardThumbnail } from "@/components/articles/content-card-thumbnail";

/**
 * Client half of a content hub (/articles, /companies), the Rustify
 * `blog_listing` layout: sidebar (search + CATEGORIES chips + reset + count) and
 * a 3-col card grid + "Load more". Pure client-side filtering over the full list
 * handed down by the server page. Chips are multi-select (OR); search matches
 * title/description on top.
 *
 * Taxonomy-agnostic: the caller passes its own closed category keys as plain
 * strings and the hub root as `basePath`. Reader behavior is not tracked.
 * Lifted from the old app/(seo)/articles/articles-listing.tsx.
 */

export type ContentListItem = {
  slug: string;
  title: string;
  shortTitleThumbnail: string;
  description: string;
  image?: string;
  category: string;
  /** already-resolved display label for the card pill */
  categoryText: string;
};

export type ContentCategoryChip = {
  key: string;
  label: string;
  count: number;
};

const PAGE_SIZE = 9;

const CARD_SHADOW = "border border-black/[0.05] shadow-md";

function ContentCard({ item, basePath }: { item: ContentListItem; basePath: string }) {
  return (
    <Link
      href={`${basePath}/${item.slug}` as Route}
      className={`group block rounded-[20px] bg-white p-2 transition-transform duration-200 hover:-translate-y-0.5 ${CARD_SHADOW}`}
    >
      <div className="relative aspect-[1200/630] w-full overflow-hidden rounded-[12px] bg-black/[0.03]">
        <ContentCardThumbnail
          title={item.shortTitleThumbnail}
          image={item.image}
          size="grid"
        />
      </div>
      <div className="flex flex-col gap-2 px-3 pb-3 pt-4">
        <span className="w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[12px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary">
          {item.categoryText}
        </span>
        <h2 className="line-clamp-3 text-[20px] font-semibold leading-[1.1] tracking-[-0.042em] text-[#0f0f10]">
          {item.title}
        </h2>
        <p className="line-clamp-2 text-[15px] leading-[1.45] text-[rgba(15,15,16,0.73)]">
          {item.description}
        </p>
      </div>
    </Link>
  );
}

export function ContentHubListing({
  items,
  chips,
  basePath,
  searchPlaceholder,
  emptyLabel,
  initialCategory = null,
}: {
  items: ContentListItem[];
  chips: ContentCategoryChip[];
  basePath: string;
  searchPlaceholder: string;
  emptyLabel: string;
  /** seeds the selected chip set from `?category=<key>`; null when absent */
  initialCategory?: string | null;
}) {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<Set<string>>(
    () => new Set(initialCategory ? [initialCategory] : []),
  );
  const [visible, setVisible] = useState(PAGE_SIZE);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return items.filter((a) => {
      if (selected.size > 0 && !selected.has(a.category)) return false;
      if (!q) return true;
      return a.title.toLowerCase().includes(q) || a.description.toLowerCase().includes(q);
    });
  }, [items, query, selected]);

  const shown = filtered.slice(0, visible);
  const hasMore = visible < filtered.length;
  const isFiltering = query.trim().length > 0 || selected.size > 0;
  function pickCategory(key: string) {
    setSelected((cur) => {
      const next = new Set(cur);
      if (next.has(key)) next.delete(key);
      else next.add(key);
      return next;
    });
    setVisible(PAGE_SIZE);
  }

  function reset() {
    setQuery("");
    setSelected(new Set());
    setVisible(PAGE_SIZE);
  }

  function loadMore() {
    const next = Math.min(visible + PAGE_SIZE, filtered.length);
    setVisible(next);
  }

  return (
    <div className="mt-16 grid gap-8 lg:grid-cols-[285px_minmax(0,1fr)]">
      <aside className="flex h-max flex-col gap-6 lg:sticky lg:top-[122px]">
        <div className="relative flex items-center rounded-[16px] border border-black/[0.08] bg-white shadow-sm">
          <Search
            className="pointer-events-none absolute left-3 size-4 text-[rgba(15,15,16,0.4)]"
            aria-hidden
          />
          <input
            type="search"
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setVisible(PAGE_SIZE);
            }}
            placeholder={searchPlaceholder}
            className="h-[42px] w-full rounded-[16px] bg-transparent pl-[38px] pr-4 text-[14px] text-[#0f0f10] outline-none placeholder:text-[rgba(15,15,16,0.4)]"
          />
        </div>

        <div className="flex flex-col gap-3">
          <p className="border-b border-black/[0.065] pb-2 text-[12px] font-medium uppercase tracking-[-0.01em] text-[rgba(15,15,16,0.5)]">
            Categories
          </p>
          <Chips items={chips} selected={selected} onToggle={pickCategory} />
        </div>

        <div className="flex items-center justify-between">
          <button
            type="button"
            onClick={reset}
            disabled={!isFiltering}
            className="text-[13px] font-medium text-[rgba(15,15,16,0.73)] underline-offset-2 hover:underline disabled:opacity-40 disabled:no-underline"
          >
            Reset filters
          </button>
          <span className="text-[13px] tabular-nums text-[rgba(15,15,16,0.5)]">
            {filtered.length} of {items.length}
          </span>
        </div>
      </aside>

      <div className="flex flex-col gap-8">
        {shown.length > 0 ? (
          <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
            {shown.map((a) => (
              <ContentCard key={a.slug} item={a} basePath={basePath} />
            ))}
          </div>
        ) : (
          <p className="py-16 text-center text-[15px] text-[rgba(15,15,16,0.5)]">
            {emptyLabel}
          </p>
        )}

        {hasMore ? (
          <div className="flex justify-center">
            <button
              type="button"
              onClick={loadMore}
              className="rounded-[10px] border border-black/[0.12] bg-white px-4 py-1.5 text-[14px] font-medium text-[#333] transition-colors hover:bg-black/[0.03]"
            >
              Load more
            </button>
          </div>
        ) : null}
      </div>
    </div>
  );
}
