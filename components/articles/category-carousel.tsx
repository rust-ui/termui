"use client";

import { BookOpen, Boxes, Cog, Rocket, type LucideIcon } from "lucide-react";

import { HubCarousel, type HubCarouselItem } from "@/components/articles/hub-carousel";
import type { ArticleCategory } from "@/domains/articles/categories";

/**
 * "Browse Categories" — the /articles hub's rail. Thin wrapper over the shared
 * <HubCarousel>: resolves the closed `ArticleCategory` enum to a lucide icon
 * here (client side) and links each tile to the dedicated
 * /articles/category/[category] page.
 */

const ICONS: Record<ArticleCategory, LucideIcon> = {
  building: BookOpen,
  widgets: Boxes,
  engineering: Cog,
  production: Rocket,
};

export type CategoryCard = {
  key: ArticleCategory;
  label: string;
  blurb: string;
  count: number;
};

export function CategoryCarousel({ categories }: { categories: CategoryCard[] }) {
  const items: HubCarouselItem[] = categories.map((c) => ({
    key: c.key,
    label: c.label,
    blurb: c.blurb,
    count: c.count,
    href: `/articles/category/${c.key}`,
    Icon: ICONS[c.key],
  }));

  return <HubCarousel kicker="All categories" title="Browse Categories" items={items} />;
}
