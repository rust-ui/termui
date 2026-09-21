import { ROUTES } from "./routes";

export interface LabsNavLink {
  href: string;
  name: string;
  description?: string;
}

export const LABS_LATEST = {
  description: "Explore the Term/UI source code and releases",
  href: "https://github.com/rust-ui/termui",
  name: "Term/UI on GitHub",
} as const satisfies LabsNavLink;

export const LABS_NAV_SECTIONS: {
  id: string;
  items: readonly LabsNavLink[];
  title: string;
}[] = [];

export const TOP_LEVEL_SECTIONS = [
  { href: ROUTES.DOCS, name: "Introduction" },
  { href: ROUTES.DOCS_INSTALLATION, name: "Installation" },
  { href: ROUTES.DOCS_MCP, name: "MCP" },
  { href: ROUTES.LLMS, name: "llms.txt" },
  { href: ROUTES.DOCS_CHANGELOG, name: "Changelog" },
];
