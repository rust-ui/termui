import { ROUTES } from "./routes";
import { RATATUI_WIDGETS_TITLE } from "./ratatui";

export interface LabsNavLink {
  href: string;
  name: string;
  description?: string;
}

export const LABS_LATEST = {
  description: "Term/UI is part of the Rust/UI ecosystem",
  href: "https://rust-ui.com",
  name: "Rust/UI",
} as const satisfies LabsNavLink;

export const LABS_REGISTRIES = [
  { href: "https://rust-ui.com/docs/registry", name: "Term/UI registry" },
] as const satisfies readonly LabsNavLink[];

export const LABS_NAV_SECTIONS = [
  { id: "registries", items: LABS_REGISTRIES, title: "Term/UI" },
] as const;

export const TOP_LEVEL_SECTIONS = [
  { href: ROUTES.DOCS, name: "Introduction" },
  { href: ROUTES.DOCS_WIDGETS, name: RATATUI_WIDGETS_TITLE },
  { href: ROUTES.DOCS_INSTALLATION, name: "Installation" },
  { href: ROUTES.DOCS_MCP, name: "MCP" },
  { href: ROUTES.DOCS_REGISTRY, name: "Registry" },
  { href: ROUTES.LLMS, name: "llms.txt" },
  { href: ROUTES.DOCS_CHANGELOG, name: "Changelog" },
];
