import type { MetadataRoute } from "next";

import { ROUTES } from "@/shared/config/routes";
import { CHART_FAMILIES } from "@/domains/charts/config";
import { SITE } from "@/shared/config/site";
import { source } from "@/domains/docs/source";

export default function sitemap(): MetadataRoute.Sitemap {
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

  return [...staticPages, ...docPages, ...chartPages];
}
