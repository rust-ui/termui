import { ImageResponse } from "next/og";

import { getAuthorBySlug } from "@/domains/articles/authors";

export const runtime = "nodejs";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default async function OpenGraphImage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const author = getAuthorBySlug(slug);
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
      <div style={{ color: "#f97316", fontSize: 28 }}>TERM/UI AUTHOR</div>
      <div style={{ fontSize: 72, fontWeight: 700, marginTop: 24 }}>
        {author?.name ?? "Term/UI contributor"}
      </div>
      <div style={{ color: "#a3a3a3", fontSize: 30, marginTop: 32 }}>
        {author?.role ?? "Ratatui and Rust terminal UI"}
      </div>
    </div>,
    size,
  );
}
