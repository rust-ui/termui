import { cn } from "@/lib/utils";

export const LogoMark = ({ className }: { className?: string }) => (
  <svg
    viewBox="0 0 64 64"
    className={cn("size-6", className)}
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    aria-hidden
  >
    <rect x="4" y="4" width="56" height="56" rx="8" className="fill-foreground" />
    <rect x="14" y="20" width="8" height="8" className="fill-background" />
    <rect x="22" y="28" width="8" height="8" className="fill-background" />
    <rect x="14" y="36" width="8" height="8" className="fill-background" />
    <rect x="34" y="42" width="16" height="6" className="fill-background" />
  </svg>
);

export const LogoType = ({ className }: { className?: string }) => (
  <span className={cn("font-mono text-base font-semibold tracking-tight", className)}>termui</span>
);
