import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "termui",
  description: "Terminal UI components, rendered by real Ratatui, live in the browser via WASM.",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en" className="dark">
      <body>{children}</body>
    </html>
  );
}
