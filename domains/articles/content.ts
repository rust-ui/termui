import { cache } from "react";

import { getAuthorByName } from "@/domains/articles/authors";
import type { ArticleCategory } from "@/domains/articles/categories";
import { articleSource } from "@/domains/articles/source";
import { tocTitleToText } from "@/domains/articles/toc";

export type ArticleSummary = {
  slug: string;
  url: string;
  title: string;
  description: string;
  keywords: string[];
  category: ArticleCategory;
  order: number;
  publishDate: string;
  lastUpdated?: string;
  author: string;
  authorImage: string;
  shortTitleThumbnail: string;
  image?: string;
  imageAlt?: string;
  readingTimeMinutes: number;
};

export type Article = ArticleSummary & {
  headings: { id: string; text: string; level: number }[];
  body: NonNullable<ReturnType<typeof articleSource.getPage>>["data"]["body"];
};

const readingTime = (text: string) =>
  Math.max(1, Math.ceil(text.trim().split(/\s+/).length / 220));

const toSummary = async (
  page: ReturnType<typeof articleSource.getPages>[number],
): Promise<ArticleSummary> => {
  const data = page.data;
  const raw = await data.getText("raw");
  return {
    slug: page.slugs[0] ?? "",
    url: page.url,
    title: data.title,
    description: data.description,
    keywords: data.keywords,
    category: data.category,
    order: data.order,
    publishDate: data.publish_date,
    ...(data.last_updated ? { lastUpdated: data.last_updated } : {}),
    author: data.author,
    authorImage: data.author_image,
    shortTitleThumbnail: data.short_title_thumbnail,
    ...(data.image ? { image: data.image } : {}),
    ...(data.image_alt ? { imageAlt: data.image_alt } : {}),
    readingTimeMinutes: readingTime(raw),
  };
};

export const getArticleSummaries = cache(async () => {
  const summaries = await Promise.all(articleSource.getPages().map(toSummary));
  return summaries.sort(
    (a, b) =>
      b.publishDate.localeCompare(a.publishDate) ||
      a.order - b.order ||
      a.slug.localeCompare(b.slug),
  );
});

export const getAllArticles = getArticleSummaries;

export const getArticleBySlug = cache(async (slug: string): Promise<Article | null> => {
  const page = articleSource.getPage([slug]);
  if (!page) return null;
  const summary = (await getArticleSummaries()).find((article) => article.slug === slug);
  if (!summary) return null;
  return {
    ...summary,
    headings: page.data.toc.map((heading) => ({
      id: heading.url.slice(1),
      text:
        tocTitleToText(heading.title) ||
        heading.url
          .slice(1)
          .split("-")
          .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
          .join(" "),
      level: heading.depth,
    })),
    body: page.data.body,
  };
});

export const getArticlesByCategory = async (category: ArticleCategory) =>
  (await getArticleSummaries()).filter((article) => article.category === category);

export const getArticleAuthor = (name: string) => getAuthorByName(name);
