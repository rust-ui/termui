import { buttonVariants } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { LINK } from "@/constants/links";
import { cn } from "@/lib/utils";

export const GitHubStars = ({ stargazersCount }: { stargazersCount: number }) => (
  <Tooltip>
    <TooltipTrigger asChild>
      <a
        href={LINK.GITHUB}
        target="_blank"
        rel="noopener"
        className={cn(buttonVariants({ size: "sm", variant: "ghost" }))}
      >
        <svg viewBox="0 0 24 24" className="size-4 -translate-y-px" fill="currentColor" aria-hidden>
          <path d="M12 .5C5.65.5.5 5.65.5 12c0 5.09 3.29 9.4 7.86 10.93.57.1.79-.25.79-.55 0-.27-.01-1.17-.02-2.12-3.2.7-3.87-1.36-3.87-1.36-.53-1.33-1.29-1.69-1.29-1.69-1.05-.72.08-.7.08-.7 1.17.08 1.78 1.2 1.78 1.2 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.55-.29-5.23-1.28-5.23-5.68 0-1.26.45-2.29 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.18 1.18a11.05 11.05 0 0 1 5.8 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.8 1.19 1.83 1.19 3.09 0 4.41-2.69 5.38-5.25 5.67.41.36.78 1.06.78 2.15 0 1.55-.01 2.8-.01 3.18 0 .31.21.66.79.55A10.51 10.51 0 0 0 23.5 12c0-6.35-5.15-11.5-11.5-11.5Z" />
        </svg>
        <span className="text-muted-foreground text-xs tabular-nums">
          {new Intl.NumberFormat("en-US", { compactDisplay: "short", notation: "compact" })
            .format(stargazersCount)
            .toLowerCase()}
        </span>
      </a>
    </TooltipTrigger>
    <TooltipContent>{new Intl.NumberFormat("en-US").format(stargazersCount)} stars</TooltipContent>
  </Tooltip>
);
