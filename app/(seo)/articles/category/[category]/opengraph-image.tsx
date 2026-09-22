import { ImageResponse } from "next/og";

import {
  ARTICLE_CATEGORY_LABELS,
  isArticleCategory,
} from "@/domains/articles/categories";

export const runtime = "nodejs";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default async function OpenGraphImage({
  params,
}: {
  params: Promise<{ category: string }>;
}) {
  const { category } = await params;
  const label = isArticleCategory(category)
    ? ARTICLE_CATEGORY_LABELS[category]
    : "Ratatui";
  return new ImageResponse(
    <div
      style={{
        background: "#0a0a0a",
        color: "#fff",
        display: "flex",
        flexDirection: "column",
        height: "100%",
        justifyContent: "center",
        padding: 80,
        width: "100%",
      }}
    >
      <div style={{ color: "#f97316", fontSize: 28 }}>TERM/UI · RUST TUI</div>
      <div style={{ fontSize: 72, fontWeight: 700, marginTop: 24 }}>{label} articles</div>
    </div>,
    size,
  );
}
