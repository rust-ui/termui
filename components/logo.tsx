import { cn } from "@/lib/utils";

export const LogoMark = ({
  className,
  ...props
}: React.ComponentProps<"svg">) => (
  <svg
    aria-hidden="true"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    className={cn("size-4 text-foreground", className)}
    {...props}
  >
    <rect
      x="2.75"
      y="3.75"
      width="18.5"
      height="16.5"
      rx="2.5"
      stroke="currentColor"
      strokeWidth="1.8"
    />
    <path
      d="m7 9 3 3-3 3m6 0h4"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="1.8"
    />
  </svg>
);

export const LogoType = ({
  className,
  ...props
}: React.ComponentProps<"svg">) => (
  <svg
    aria-label="Term/UI"
    role="img"
    viewBox="0 0 184 32"
    xmlns="http://www.w3.org/2000/svg"
    className={cn("h-4 w-auto fill-foreground", className)}
    {...props}
  >
    <text
      x="0"
      y="25"
      fontFamily="ui-monospace, SFMono-Regular, Menlo, monospace"
      fontSize="25"
      fontWeight="700"
      letterSpacing="-1.5"
    >
      Term/UI
    </text>
  </svg>
);

export const getLogoMarkSVG = (color: string) => `
<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  <rect x="2.75" y="3.75" width="18.5" height="16.5" rx="2.5" stroke="${color}" stroke-width="1.8" />
  <path d="m7 9 3 3-3 3m6 0h4" stroke="${color}" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" />
</svg>
`;

export const getLogoTypeSVG = (color: string) => `
<svg viewBox="0 0 184 32" xmlns="http://www.w3.org/2000/svg">
  <text x="0" y="25" fill="${color}" font-family="ui-monospace, SFMono-Regular, Menlo, monospace" font-size="25" font-weight="700" letter-spacing="-1.5">Term/UI</text>
</svg>
`;
