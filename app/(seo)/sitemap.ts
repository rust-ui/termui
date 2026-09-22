import type { MetadataRoute } from "next";

import { ROUTES } from "@/shared/config/routes";
import { CHART_FAMILIES } from "@/domains/charts/config";
import { SITE } from "@/shared/config/site";
import { source } from "@/domains/docs/source";
import { ARTICLE_CATEGORIES } from "@/domains/articles/categories";
import { getArticleSummaries } from "@/domains/articles/content";

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const staticPages: MetadataRoute.Sitemap = [
    {
      changeFrequency: "monthly",
      priority: 1,
      url: SITE.URL,
    },
  ];

  const docPages: MetadataRoute.Sitemap = source.getPages().map((page) => ({
    changeFrequency: "weekly" as const,
    priority: page.url === ROUTES.DOCS ? 0.9 : 0.8,
    url: `${SITE.URL}${page.url}`,
  }));

  const chartPages: MetadataRoute.Sitemap = CHART_FAMILIES.map(
    (family) => `${ROUTES.CHARTS}/${family.type}`,
  ).map((path) => ({
    changeFrequency: "monthly",
    priority: 0.8,
    url: `${SITE.URL}${path}`,
  }));

  const articlePages = (await getArticleSummaries()).map((article) => ({
    changeFrequency: "monthly" as const,
    lastModified: new Date(article.lastUpdated ?? article.publishDate),
    priority: article.order === 1 ? 0.9 : 0.7,
    url: `${SITE.URL}${article.url}`,
  }));
  const categoryPages = ARTICLE_CATEGORIES.map((category) => ({
    changeFrequency: "monthly" as const,
    priority: 0.7,
    url: `${SITE.URL}${ROUTES.ARTICLES}/category/${category}`,
  }));
  const editorialPages = [
    {
      changeFrequency: "weekly" as const,
      priority: 0.8,
      url: `${SITE.URL}${ROUTES.ARTICLES}`,
    },
    {
      changeFrequency: "monthly" as const,
      priority: 0.5,
      url: `${SITE.URL}${ROUTES.AUTHORS}`,
    },
    {
      changeFrequency: "monthly" as const,
      priority: 0.5,
      url: `${SITE.URL}${ROUTES.AUTHORS}/max-wells`,
    },
  ];

  return [
    ...staticPages,
    ...docPages,
    ...chartPages,
    ...articlePages,
    ...categoryPages,
    ...editorialPages,
  ];
}
