import type { MetadataRoute } from "next";

import { ROUTES } from "@/constants/routes";
import { CHART_FAMILIES } from "@/constants/charts";
import { SITE } from "@/constants/site";
import { source } from "@/lib/source";

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
