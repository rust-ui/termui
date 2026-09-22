export const ARTICLE_CATEGORIES = [
  "building",
  "widgets",
  "engineering",
  "production",
] as const;

export type ArticleCategory = (typeof ARTICLE_CATEGORIES)[number];

export const ARTICLE_CATEGORY_LABELS: Record<ArticleCategory, string> = {
  building: "Building",
  widgets: "Widgets",
  engineering: "Engineering",
  production: "Production",
};

export const ARTICLE_CATEGORY_DESCRIPTIONS: Record<ArticleCategory, string> = {
  building: "Start with the core layout and composition patterns for Rust terminal UIs.",
  widgets: "Choose and compose Ratatui widgets for useful terminal workflows.",
  engineering: "Make input, async work, and rendering behavior reliable and testable.",
  production: "Make terminal interfaces usable, portable, and ready to ship.",
};

export const isArticleCategory = (value: string): value is ArticleCategory =>
  ARTICLE_CATEGORIES.includes(value as ArticleCategory);
