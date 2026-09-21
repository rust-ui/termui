import Link from "next/link";
import { ArrowRightIcon } from "lucide-react";

import { ChartsNav } from "@/domains/charts/components/charts/charts-nav";
import { PageHero } from "@/domains/site/components/page-hero";
import { Button } from "@/components/ui/button";

export default function ChartsLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="container-wrapper">
      <div className="container pb-16">
        <header className="mx-auto flex max-w-3xl flex-col items-center pt-16 pb-14 text-center md:pt-20">
          <Link
            className="mb-5 inline-flex items-center gap-2 rounded-full bg-muted px-3 py-1.5 text-xs font-medium transition-colors hover:bg-muted/70"
            href="/docs/widgets/chart"
          >
            New chart components <ArrowRightIcon className="size-3.5" />
          </Link>
          <PageHero
            title="Beautiful Charts & Graphs"
            titleClassName="text-4xl sm:text-5xl md:text-6xl"
            description={
              <>
                A collection of ready-to-use chart components built with Ratatui. From
                basic charts to rich data displays, copy and paste into your terminal
                apps.
              </>
            }
            descriptionClassName="max-w-2xl text-base md:text-lg"
          />
          <div className="mt-7 flex items-center gap-2">
            <Button asChild className="rounded-full" size="sm">
              <Link href="#charts">Browse Charts</Link>
            </Button>
            <Button asChild className="rounded-full" size="sm" variant="secondary">
              <Link href="/docs/widgets/chart">Documentation</Link>
            </Button>
          </div>
        </header>
        <ChartsNav />
        {children}
      </div>
    </div>
  );
}
