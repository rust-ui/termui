"use client";

import Link from "next/link";
import { ArrowUpRightIcon } from "lucide-react";

import { Button } from "@/components/ui/button";
import { useMediaQuery } from "@/hooks/use-media-query";
import { cn } from "@/lib/utils";

interface DocsRustifyCtaProps {
  slot: "content" | "sidebar";
  className?: string;
}

export const DocsRustifyCta = ({ slot, className }: DocsRustifyCtaProps) => {
  const hasSidebar = useMediaQuery("(min-width: 1280px)");

  if (hasSidebar === undefined || hasSidebar !== (slot === "sidebar")) {
    return null;
  }

  return (
    <div
      className={cn(
        "group relative flex flex-col gap-2 rounded-2xl border bg-muted p-6 text-sm text-card-foreground shadow-sm",
        className,
      )}
      data-slot="docs-rustify-cta"
    >
      <div className="text-base leading-tight font-semibold text-balance group-hover:underline">
        Learn Rust with Rustify.rs
      </div>
      <div className="text-muted-foreground">
        Practical guides and tools for your next Rust project.
      </div>
      <Button className="mt-2 w-fit" size="sm" variant="outline">
        Learn Rust <ArrowUpRightIcon />
      </Button>
      <Link
        className="absolute inset-0"
        href="https://rustify.rs"
        target="_blank"
        rel="noreferrer"
      >
        <span className="sr-only">Learn Rust with Rustify.rs</span>
      </Link>
    </div>
  );
};
