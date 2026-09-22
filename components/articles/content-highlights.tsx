import Link from "next/link";
import type { Route } from "next";

import { ContentCardThumbnail } from "@/components/articles/content-card-thumbnail";
import { PanelLabel } from "@/components/articles/panel-label";

/**
 * Server-rendered highlight cards for content hubs (/articles, /companies) and
 * their [category] pages. Lifted 1:1 from the old
 * app/(seo)/articles/article-highlights.tsx (itself from app/wip/blog/page.tsx)
 * so every hub stays visually identical.
 *
 * Taxonomy-agnostic: the caller resolves its own closed category enum to a
 * display string (`categoryText`) and passes the hub root as `basePath`
 * (`/articles`, `/companies`). The pill uses the `primary` colour utilities
 * (Rustify brand orange, `--primary` in src/styles.css). `PanelLabel` is shared
 * from my-components and re-exported here for import parity.
 */

const CARD_SHADOW = "border border-black/[0.05] shadow-md";

export { PanelLabel };

export type ContentHighlightItem = {
  slug: string;
  title: string;
  /** label for the generated card thumbnail (article short title, company name, ...) */
  shortTitleThumbnail: string;
  description: string;
  image?: string;
  /** already-resolved display label for the card pill */
  categoryText: string;
};

export function FeaturedCard({
  item,
  basePath,
}: {
  item: ContentHighlightItem;
  basePath: string;
}) {
  return (
    <Link
      href={`${basePath}/${item.slug}` as Route}
      className={`group block rounded-[20px] bg-white p-2 transition-transform duration-200 hover:-translate-y-0.5 ${CARD_SHADOW}`}
    >
      <div className="relative aspect-[1200/630] w-full overflow-hidden rounded-[12px] bg-black/[0.03]">
        <ContentCardThumbnail
          title={item.shortTitleThumbnail}
          image={item.image}
          size="feature"
          priority
        />
      </div>
      <div className="grid gap-4 p-4 sm:grid-cols-2">
        <div className="flex flex-col gap-2">
          <span className="w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[12px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary">
            {item.categoryText}
          </span>
          <h2 className="text-[28px] font-semibold leading-[1.1] tracking-[-0.042em] text-[#0f0f10]">
            {item.title}
          </h2>
        </div>
        <p className="line-clamp-4 self-center text-[15px] leading-[1.5] text-[rgba(15,15,16,0.73)]">
          {item.description}
        </p>
      </div>
    </Link>
  );
}

export function SpotlightRow({
  item,
  basePath,
}: {
  item: ContentHighlightItem;
  basePath: string;
}) {
  return (
    <Link
      href={`${basePath}/${item.slug}` as Route}
      className={`grid grid-cols-[128px_minmax(0,1fr)] items-stretch gap-0 rounded-[20px] bg-white p-2 transition-transform duration-200 hover:-translate-y-0.5 ${CARD_SHADOW}`}
    >
      <div className="relative aspect-[16/10] overflow-hidden rounded-[12px] bg-black/[0.03]">
        <ContentCardThumbnail
          title={item.shortTitleThumbnail}
          image={item.image}
          size="spot"
        />
      </div>
      <div className="flex flex-col justify-start gap-1.5 pl-4 pr-2 pt-1">
        <span className="w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[11px] font-medium uppercase leading-[1.4] tracking-[-0.01em] text-primary">
          {item.categoryText}
        </span>
        <h2 className="line-clamp-2 text-[15px] font-semibold leading-[1.3] tracking-[-0.03em] text-[#0f0f10]">
          {item.title}
        </h2>
      </div>
    </Link>
  );
}
