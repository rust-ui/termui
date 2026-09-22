"use client";

import { useEffect, useState } from "react";

import { GithubIcon } from "@/components/shared/icons";
import { buttonVariants } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { LINK } from "@/shared/config/links";
import { UTM_PARAMS } from "@/shared/config/site";
import { useFeedback } from "@/shared/hooks/use-feedback";
import { addQueryParams } from "@/shared/lib/url";
import { cn } from "@/shared/lib/utils";

export const GitHubStars = () => {
  const [stargazersCount, setStargazersCount] = useState<number | null>(null);
  const play = useFeedback({ sound: "star" });

  useEffect(() => {
    const controller = new AbortController();

    const loadCount = async () => {
      try {
        const response = await fetch("/api/github-stars", {
          signal: controller.signal,
        });

        if (!response.ok) {
          return;
        }

        const json = (await response.json()) as { count?: unknown };
        if (typeof json.count === "number" && Number.isInteger(json.count)) {
          setStargazersCount(json.count);
        }
      } catch {
        // Keep the loading placeholder when the public API is unavailable.
      }
    };

    void loadCount();
    return () => controller.abort();
  }, []);

  const formattedCount =
    stargazersCount === null
      ? null
      : new Intl.NumberFormat("en-US", {
          compactDisplay: "short",
          notation: "compact",
        })
          .format(stargazersCount)
          .toLowerCase();

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <a
          href={addQueryParams(LINK.GITHUB, UTM_PARAMS)}
          target="_blank"
          rel="noopener"
          aria-label="GitHub repository"
          onClick={play}
          className={cn(buttonVariants({ size: "sm", variant: "ghost" }))}
        >
          <GithubIcon className="-translate-y-px" />
          {formattedCount === null ? (
            <Skeleton className="h-3 w-6 rounded-sm" aria-label="Loading GitHub stars" />
          ) : (
            <span className="text-xs text-muted-foreground tabular-nums">
              {formattedCount}
            </span>
          )}
        </a>
      </TooltipTrigger>
      <TooltipContent>
        {stargazersCount === null
          ? "GitHub stars"
          : `${new Intl.NumberFormat("en-US").format(stargazersCount)} stars`}
      </TooltipContent>
    </Tooltip>
  );
};
