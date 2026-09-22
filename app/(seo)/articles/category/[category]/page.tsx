import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";

import {
  ContentCategoryGrid,
  type ContentCategoryGridItem,
} from "@/components/articles/content-category-grid";
import { ArticleClosingCta } from "@/components/articles/closing-cta";
import {
  ARTICLE_CATEGORIES,
  ARTICLE_CATEGORY_DESCRIPTIONS,
  ARTICLE_CATEGORY_LABELS,
  isArticleCategory,
} from "@/domains/articles/categories";
import { getArticleSummaries } from "@/domains/articles/content";
import { BreadcrumbJsonLd, CollectionPageJsonLd } from "@/app/(seo)/_lib/json-ld";
import { createPageMetadata } from "@/app/(seo)/_lib/metadata";
import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

export const dynamicParams = false;

export function generateStaticParams() {
  return ARTICLE_CATEGORIES.map((category) => ({ category }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ category: string }>;
}): Promise<Metadata> {
  const { category } = await params;
  if (!isArticleCategory(category)) return {};
  return createPageMetadata({
    description: ARTICLE_CATEGORY_DESCRIPTIONS[category],
    ogImage: `${SITE.URL}${ROUTES.ARTICLES}/category/${category}/opengraph-image`,
    path: `${ROUTES.ARTICLES}/category/${category}`,
    title: `${ARTICLE_CATEGORY_LABELS[category]} Ratatui Articles`,
  });
}

export default async function ArticleCategoryPage({
  params,
}: {
  params: Promise<{ category: string }>;
}) {
  const { category } = await params;
  if (!isArticleCategory(category)) notFound();
  const summaries = await getArticleSummaries();
  const inCategory = summaries.filter((article) => article.category === category);
  if (inCategory.length === 0) notFound();
  const articles: ContentCategoryGridItem[] = inCategory.map((article) => ({
    slug: article.slug,
    title: article.title,
    shortTitleThumbnail: article.shortTitleThumbnail,
    description: article.description,
    image: article.image,
    categoryText: ARTICLE_CATEGORY_LABELS[article.category],
  }));
  const path = `${ROUTES.ARTICLES}/category/${category}`;
  return (
    <>
      <BreadcrumbJsonLd
        items={[
          { name: "Home", path: ROUTES.HOME },
          { name: "Articles", path: ROUTES.ARTICLES },
          { name: ARTICLE_CATEGORY_LABELS[category], path },
        ]}
      />
      <CollectionPageJsonLd
        name={`${ARTICLE_CATEGORY_LABELS[category]} Ratatui Articles`}
        description={ARTICLE_CATEGORY_DESCRIPTIONS[category]}
        url={`${SITE.URL}${path}`}
        items={articles.map((article) => ({
          name: article.title,
          url: `${SITE.URL}${ROUTES.ARTICLES}/${article.slug}`,
        }))}
      />
      <div className="mx-auto max-w-[1200px] px-6 pb-24 pt-16 md:pt-24">
        <nav className="flex items-center gap-2" aria-label="Breadcrumb">
          <Link
            href={ROUTES.ARTICLES}
            className="rounded-full border bg-black/[0.04] px-2.5 py-1 text-[11px] font-medium uppercase"
          >
            Articles
          </Link>
          <span className="text-sm text-muted-foreground">/</span>
          <span className="rounded-full border border-primary/30 bg-primary/10 px-2.5 py-1 text-[11px] font-medium uppercase text-primary">
            {ARTICLE_CATEGORY_LABELS[category]}
          </span>
        </nav>
        <header className="mt-6 flex max-w-[560px] flex-col gap-4">
          <h1 className="text-[clamp(40px,6vw,56px)] font-semibold leading-[1.03] tracking-[-0.045em]">
            {ARTICLE_CATEGORY_LABELS[category]}
          </h1>
          <p className="text-[17px] leading-[1.45] text-muted-foreground">
            {ARTICLE_CATEGORY_DESCRIPTIONS[category]}
          </p>
        </header>
        <ContentCategoryGrid items={articles} basePath={ROUTES.ARTICLES} />
        <ArticleClosingCta />
      </div>
    </>
  );
}
