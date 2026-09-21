import Link from "next/link";

import { isWidgetsFolder } from "@/lib/docs";
import type { PageTreeFolder, PageTreePage } from "@/lib/page-tree";
import { getFolderSections } from "@/lib/page-tree";
import { source } from "@/lib/source";
import { cn } from "@/lib/utils";

const widgetsFolder = source.pageTree.children.find(
  (node): node is PageTreeFolder =>
    node.type === "folder" && isWidgetsFolder(node)
);

const ComponentGrid = ({
  className,
  pages,
}: {
  className?: string;
  pages: PageTreePage[];
}) => (
  <div
    className={cn(
      "grid grid-cols-2 gap-4 md:grid-cols-3 md:gap-x-8 lg:gap-x-16 lg:gap-y-6 xl:gap-x-20",
      className
    )}
  >
    {pages.map((component) => (
      <Link
        key={component.$id}
        href={component.url}
        className="inline-flex items-center gap-2 text-lg font-medium underline-offset-4 hover:underline md:text-base"
        transitionTypes={["nav-forward"]}
      >
        {component.name}
      </Link>
    ))}
  </div>
);

export const WidgetsList = ({ className }: { className?: string }) => {
  const folder = widgetsFolder;
  if (!folder) {
    return null;
  }

  const pages = getFolderSections(folder).flatMap((section) => section.pages);

  return pages.length > 0 ? (
    <ComponentGrid className={className} pages={pages} />
  ) : null;
};
