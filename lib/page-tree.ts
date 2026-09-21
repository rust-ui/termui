import type {
  Node as PageTreeNode,
  Root as PageTreeRoot,
} from "fumadocs-core/page-tree";

import { ROUTES } from "@/constants/routes";
import { RATATUI_WIDGETS_TITLE } from "@/constants/ratatui";
import { EXCLUDED_SECTIONS, isWidgetsFolder } from "@/lib/docs";

export type PageTreeFolder = Extract<PageTreeNode, { type: "folder" }>;
export type PageTreePage = Extract<PageTreeNode, { type: "page" }>;

export interface TreeGroup {
  label: string;
  pages: PageTreePage[];
}

export interface FolderSection extends TreeGroup {
  id: string;
}

export const getAllPagesFromFolder = (
  folder: PageTreeFolder
): PageTreePage[] => {
  const pages: PageTreePage[] = [];

  for (const child of folder.children) {
    if (child.type === "page") {
      pages.push(child);
    } else if (child.type === "folder") {
      pages.push(...getAllPagesFromFolder(child));
    }
  }

  return pages;
};

export const getFolderPages = (folder: PageTreeFolder): PageTreePage[] =>
  getAllPagesFromFolder(folder);

export const getFolderSections = (
  folder: PageTreeFolder
): FolderSection[] => {
  if (isWidgetsFolder(folder)) {
    const pages = getFolderPages(folder).filter(
      (page) => page.url !== ROUTES.DOCS_WIDGETS
    );
    return pages.length > 0
      ? [{ id: ROUTES.DOCS_WIDGETS, label: RATATUI_WIDGETS_TITLE, pages }]
      : [];
  }
  return [];
};

export const getTreeGroups = (
  tree: PageTreeRoot
): TreeGroup[] => {
  const groups: TreeGroup[] = [];

  for (const item of tree.children) {
    if (item.type !== "folder") {
      continue;
    }
    if (EXCLUDED_SECTIONS.has(item.$id ?? "")) {
      continue;
    }

    if (isWidgetsFolder(item)) {
      for (const section of getFolderSections(item)) {
        groups.push({
          label: section.label,
          pages: section.pages,
        });
      }
    } else {
      const pages = getFolderPages(item);
      if (pages.length > 0) {
        groups.push({
          label: typeof item.name === "string" ? item.name : String(item.name),
          pages,
        });
      }
    }
  }

  return groups;
};
