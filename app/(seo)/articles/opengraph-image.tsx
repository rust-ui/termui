import { ImageResponse } from "next/og";

export const runtime = "nodejs";
export const alt = "Term/UI Ratatui and Rust TUI articles";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default function OpenGraphImage() {
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
      <div style={{ color: "#f97316", fontSize: 28 }}>TERM/UI · RUST TERMINAL UI</div>
      <div style={{ fontSize: 72, fontWeight: 700, marginTop: 24 }}>
        Ratatui and Rust TUI guides
      </div>
      <div style={{ color: "#a3a3a3", fontSize: 30, marginTop: 32 }}>
        Build practical terminal interfaces in Rust.
      </div>
    </div>,
    size,
  );
}
