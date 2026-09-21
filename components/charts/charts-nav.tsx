"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

import { CHART_FAMILIES } from "@/constants/charts";
import { ROUTES } from "@/constants/routes";
import { cn } from "@/lib/utils";

export function ChartsNav() {
  const pathname = usePathname();

  return (
    <nav
      aria-label="Chart categories"
      className="no-scrollbar -mx-2 flex overflow-x-auto border-b px-2"
    >
      {CHART_FAMILIES.map((family) => {
        const href = `${ROUTES.CHARTS}/${family.type}`;
        const active = pathname === href;

        return (
          <Link
            key={family.type}
            aria-current={active ? "page" : undefined}
            className={cn(
              "border-b-2 border-transparent px-4 py-3 text-sm whitespace-nowrap text-muted-foreground transition-colors hover:text-foreground",
              active && "border-foreground text-foreground"
            )}
            href={href}
          >
            {family.label}
          </Link>
        );
      })}
    </nav>
  );
}
