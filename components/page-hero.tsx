import type { ReactNode } from "react";

import { cn } from "@/lib/utils";

interface PageHeroProps {
  description: ReactNode;
  descriptionClassName?: string;
  title: ReactNode;
  titleClassName?: string;
}

export const PageHero = ({
  description,
  descriptionClassName,
  title,
  titleClassName,
}: PageHeroProps) => (
  <header className="relative flex flex-col items-center gap-4 text-center">
    <h1
      className={cn(
        "from-foreground via-foreground to-foreground/65 bg-linear-to-b bg-clip-text text-4xl font-bold tracking-tight text-transparent sm:text-5xl md:text-6xl",
        titleClassName
      )}
    >
      {title}
    </h1>
    <p
      className={cn(
        "text-muted-foreground max-w-lg text-base leading-relaxed text-balance",
        descriptionClassName
      )}
    >
      {description}
    </p>
  </header>
);
