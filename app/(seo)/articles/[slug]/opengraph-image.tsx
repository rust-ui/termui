import { ImageResponse } from "next/og";

import { getArticleBySlug } from "@/domains/articles/content";

export const runtime = "nodejs";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default async function OpenGraphImage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const article = await getArticleBySlug(slug);
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
      <div style={{ color: "#f97316", fontSize: 28 }}>TERM/UI · RUST TUI ARTICLE</div>
      <div style={{ fontSize: 58, fontWeight: 700, lineHeight: 1.1, marginTop: 24 }}>
        {article?.title ?? "Ratatui and Rust terminal UI"}
      </div>
      <div style={{ color: "#a3a3a3", fontSize: 28, marginTop: 32 }}>
        Practical Rust terminal UI patterns.
      </div>
    </div>,
    size,
  );
}
