import { ROUTES } from "./routes";

export interface LabsNavLink {
  href: string;
  name: string;
  description?: string;
  logo?: {
    light: string;
    dark: string;
  };
}

export const LABS_LATEST = {
  description: "Reusable components for Dioxus and Rust fullstack apps",
  href: "https://rust-ui.com",
  logo: {
    dark: "/rust-ui-logo-dark.webp",
    light: "/rust-ui-logo-light.webp",
  },
  name: "Rust/UI",
} as const satisfies LabsNavLink;

export const LABS_NAV_SECTIONS: {
  id: string;
  items: readonly LabsNavLink[];
  title: string;
}[] = [
  {
    id: "ecosystem",
    items: [
      { href: "https://rust-ui.com", name: "Rust/UI" },
      { href: "https://leptos.rust-ui.com", name: "Leptos UI" },
      { href: "https://rustify.rs", name: "Rustify" },
    ],
    title: "Ecosystem",
  },
];

export const TOP_LEVEL_SECTIONS = [
  { href: ROUTES.DOCS, name: "Introduction" },
  { href: ROUTES.CHARTS, name: "Charts" },
  { href: ROUTES.DOCS_INSTALLATION, name: "Installation" },
  { href: ROUTES.DOCS_MCP, name: "MCP" },
  { href: ROUTES.LLMS, name: "llms.txt" },
  { href: ROUTES.DOCS_CHANGELOG, name: "Changelog" },
];
