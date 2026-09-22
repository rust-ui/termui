import type { Metadata } from "next";

import { BracketHeading } from "@/components/articles/bracket-heading";
import { ArticleClosingCta } from "@/components/articles/closing-cta";
import {
  FeaturedCard,
  PanelLabel,
  SpotlightRow,
  type ContentHighlightItem,
} from "@/components/articles/content-highlights";
import {
  ContentHubListing,
  type ContentCategoryChip,
  type ContentListItem,
} from "@/components/articles/content-hub-listing";
import {
  CategoryCarousel,
  type CategoryCard,
} from "@/components/articles/category-carousel";
import {
  ARTICLE_CATEGORIES,
  ARTICLE_CATEGORY_DESCRIPTIONS,
  ARTICLE_CATEGORY_LABELS,
  isArticleCategory,
  type ArticleCategory,
} from "@/domains/articles/categories";
import { getArticleSummaries } from "@/domains/articles/content";
import { CollectionPageJsonLd } from "@/app/(seo)/_lib/json-ld";
import { createPageMetadata } from "@/app/(seo)/_lib/metadata";
import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

export async function generateMetadata({
  searchParams,
}: {
  searchParams: Promise<Record<string, string | string[] | undefined>>;
}): Promise<Metadata> {
  const params = await searchParams;
  const hasQuery = Object.values(params).some((value) => value !== undefined);
  return createPageMetadata({
    description:
      "Practical Ratatui and Rust terminal UI articles covering layouts, widgets, event loops, testing, UX, and shipping.",
    noIndex: hasQuery,
    path: ROUTES.ARTICLES,
    title: "Ratatui and Rust TUI Articles",
  });
}

export default async function ArticlesHubPage({
  searchParams,
}: {
  searchParams: Promise<{ category?: string }>;
}) {
  const summaries = await getArticleSummaries();
  const { category } = await searchParams;
  const initialCategory: ArticleCategory | null =
    category && isArticleCategory(category) ? category : null;
  const counts = new Map<ArticleCategory, number>();
  for (const article of summaries) {
    counts.set(article.category, (counts.get(article.category) ?? 0) + 1);
  }
  const items: ContentListItem[] = summaries.map((article) => ({
    slug: article.slug,
    title: article.title,
    shortTitleThumbnail: article.shortTitleThumbnail,
    description: article.description,
    image: article.image,
    category: article.category,
    categoryText: ARTICLE_CATEGORY_LABELS[article.category],
  }));
  const chips: ContentCategoryChip[] = ARTICLE_CATEGORIES.map((key) => ({
    key,
    label: ARTICLE_CATEGORY_LABELS[key],
    count: counts.get(key) ?? 0,
  }));
  const categoryCards: CategoryCard[] = ARTICLE_CATEGORIES.map((key) => ({
    key,
    label: ARTICLE_CATEGORY_LABELS[key],
    blurb: ARTICLE_CATEGORY_DESCRIPTIONS[key],
    count: counts.get(key) ?? 0,
  }));
  const toHighlight = (article: (typeof summaries)[number]): ContentHighlightItem => ({
    slug: article.slug,
    title: article.title,
    shortTitleThumbnail: article.shortTitleThumbnail,
    description: article.description,
    image: article.image,
    categoryText: ARTICLE_CATEGORY_LABELS[article.category],
  });
  const featured = summaries[0];
  const spotlight = summaries.slice(1, 5);

  return (
    <>
      <CollectionPageJsonLd
        name="Ratatui and Rust TUI Articles"
        description="Practical Ratatui and Rust terminal UI articles."
        url={`${SITE.URL}${ROUTES.ARTICLES}`}
        items={summaries.map((article) => ({
          name: article.title,
          url: `${SITE.URL}${article.url}`,
        }))}
      />
      <div className="mx-auto max-w-[1200px] px-6 pb-24 pt-16 md:pt-24">
        <header className="mx-auto flex max-w-[680px] flex-col items-center gap-4 text-center">
          <BracketHeading as="h1" align="center" kicker="Articles">
            Ratatui and Rust TUI guides
          </BracketHeading>
          <p className="mx-auto max-w-[42ch] text-balance text-lg leading-[1.4] text-muted-foreground">
            {summaries.length} practical guides across {chips.length} categories:
            building, widgets, engineering, and production.
          </p>
        </header>
        {featured ? (
          <section className="mt-16 grid gap-8 lg:grid-cols-[1.4fr_1fr]">
            <div className="rounded-[32px] bg-black/[0.02] p-6">
              <PanelLabel>Featured</PanelLabel>
              <FeaturedCard item={toHighlight(featured)} basePath={ROUTES.ARTICLES} />
            </div>
            <div className="rounded-[32px] bg-black/[0.02] p-6">
              <PanelLabel>Spotlight</PanelLabel>
              <div className="flex flex-col gap-3">
                {spotlight.map((article) => (
                  <SpotlightRow
                    key={article.slug}
                    item={toHighlight(article)}
                    basePath={ROUTES.ARTICLES}
                  />
                ))}
              </div>
            </div>
          </section>
        ) : null}
        <ContentHubListing
          items={items}
          chips={chips}
          basePath={ROUTES.ARTICLES}
          searchPlaceholder="Search articles"
          emptyLabel="No articles match that filter."
          initialCategory={initialCategory}
        />
        <CategoryCarousel categories={categoryCards} />
        <ArticleClosingCta />
      </div>
    </>
  );
}
