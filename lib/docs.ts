import { ROUTES } from "@/constants/routes";
import { RATATUI_WIDGETS_TITLE } from "@/constants/ratatui";

import type { PageTreeFolder } from "./page-tree";
import { formatLabelFromSlug } from "./utils";

export const DOCS_DIR = `content${ROUTES.DOCS}`;

export const EXCLUDED_SECTIONS = new Set(["installation", "changelog", "(root)"]);

export const isWidgetsFolder = (folder: PageTreeFolder) =>
  folder.$id === ROUTES.DOCS_WIDGETS.slice(ROUTES.DOCS.length + 1) ||
  folder.name === RATATUI_WIDGETS_TITLE;

export type DocsSidebarPanel = "widgets";

const isPathWithin = (pathname: string, route: string) =>
  pathname === route || pathname.startsWith(`${route}/`);

export const getDocsSidebarPanel = (pathname: string): DocsSidebarPanel | null => {
  if (isPathWithin(pathname, ROUTES.DOCS_WIDGETS)) {
    return "widgets";
  }
  return null;
};

const TITLE_OVERRIDES: Record<string, string> = {
  json: "JSON",
  "qr-code": "QR Code",
};

export const formatTitleFromSlug = (slug: string): string =>
  TITLE_OVERRIDES[slug] ?? formatLabelFromSlug(slug);

export const homeContentRoute = `${ROUTES.LLMS_MD}/content.md`;
export const docsContentRoute = `${ROUTES.LLMS_MD}${ROUTES.DOCS}`;

export const PAGES_NEW: string[] = [ROUTES.DOCS_CHANGELOG];
