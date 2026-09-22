import Link from "next/link";

import { ContentCardThumbnail } from "@/components/articles/content-card-thumbnail";
import { ARTICLE_CATEGORY_LABELS } from "@/domains/articles/categories";
import type { ArticleSummary } from "@/domains/articles/content";

const CARD_SHADOW = "border border-black/[0.05] shadow-md";

export function AuthorArticleCard({ article }: { article: ArticleSummary }) {
  return (
    <Link
      href={`/articles/${article.slug}`}
      className={`group block rounded-[20px] bg-white p-2 transition-transform duration-200 hover:-translate-y-0.5 ${CARD_SHADOW}`}
    >
      <div className="relative aspect-[1200/630] w-full overflow-hidden rounded-[12px] bg-black/[0.03]">
        <ContentCardThumbnail
          title={article.shortTitleThumbnail}
          image={article.image}
          size="grid"
        />
      </div>
      <div className="flex flex-col gap-2 px-3 pb-3 pt-4">
        <span className="w-fit rounded-[7px] border border-primary/30 bg-primary/10 px-1.5 py-px text-[12px] font-medium uppercase leading-[1.4] text-primary">
          {ARTICLE_CATEGORY_LABELS[article.category]}
        </span>
        <h3 className="line-clamp-3 text-[20px] font-semibold leading-[1.1] tracking-[-0.042em]">
          {article.title}
        </h3>
        <p className="line-clamp-2 text-[15px] leading-[1.4] text-muted-foreground">
          {article.description}
        </p>
      </div>
    </Link>
  );
}
