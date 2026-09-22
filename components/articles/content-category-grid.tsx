"use client";

import { useState } from "react";
import Link from "next/link";
import type { Route } from "next";

import { ContentCardThumbnail } from "@/components/articles/content-card-thumbnail";

/**
 * Client half of a content [category] page (/articles/category/*,
 * /companies/category/*): the card grid + "Load more" pager, matching the
 * Rustify blog-category layout. No search/chips (the hub owns those).
 *
 * Taxonomy-agnostic: caller passes the hub root as `basePath`, an
 * a resolved `categoryText` per card. Lifted from the old
 * app/(seo)/articles/category/[category]/category-article-grid.tsx.
 */

export type ContentCategoryGridItem = {
  slug: string;
  title: string;
  shortTitleThumbnail: string;
  description: string;
  image?: string;
  /** already-resolved display label for the card pill */
  categoryText: string;
};

const PAGE_SIZE = 9;

const CARD_SHADOW = "border border-black/[0.05] shadow-md";

function Card({ item, basePath }: { item: ContentCategoryGridItem; basePath: string }) {
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
        <h3 className="line-clamp-3 text-[20px] font-semibold leading-[1.1] tracking-[-0.042em] text-[#0f0f10]">
          {item.title}
        </h3>
        <p className="line-clamp-2 text-[15px] leading-[1.45] text-[rgba(15,15,16,0.73)]">
          {item.description}
        </p>
      </div>
    </Link>
  );
}

export function ContentCategoryGrid({
  items,
  basePath,
}: {
  items: ContentCategoryGridItem[];
  basePath: string;
}) {
  const [visible, setVisible] = useState(PAGE_SIZE);
  const shown = items.slice(0, visible);
  const hasMore = visible < items.length;

  function loadMore() {
    const next = Math.min(visible + PAGE_SIZE, items.length);
    setVisible(next);
  }

  return (
    <div className="mt-14 flex flex-col gap-8">
      <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
        {shown.map((a) => (
          <Card key={a.slug} item={a} basePath={basePath} />
        ))}
      </div>

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
  );
}
