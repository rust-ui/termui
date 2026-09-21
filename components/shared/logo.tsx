import { cn } from "@/shared/lib/utils";

const PIXEL_R_RECTS = [
  [126, 74, 208, 52],
  [126, 126, 52, 312],
  [334, 126, 52, 104],
  [126, 230, 208, 52],
  [230, 282, 52, 52],
  [282, 334, 52, 52],
  [334, 386, 52, 52],
] as const;

const PIXEL_R_SVG = PIXEL_R_RECTS.map(
  ([x, y, width, height]) =>
    `<rect x="${x}" y="${y}" width="${width}" height="${height}"/>`,
).join("\n  ");

export const LogoMark = ({ className, ...props }: React.ComponentProps<"svg">) => (
  <svg
    aria-hidden="true"
    viewBox="0 0 512 512"
    xmlns="http://www.w3.org/2000/svg"
    className={cn("size-4 text-foreground", className)}
    shapeRendering="crispEdges"
    {...props}
  >
    <rect width="512" height="512" rx="115" fill="#000" />
    <g fill="#fff">
      {PIXEL_R_RECTS.map(([x, y, width, height], index) => (
        <rect key={index} x={x} y={y} width={width} height={height} />
      ))}
    </g>
  </svg>
);

export const LogoType = ({ className, ...props }: React.ComponentProps<"svg">) => (
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
<svg viewBox="0 0 512 512" fill="${color}" xmlns="http://www.w3.org/2000/svg" shape-rendering="crispEdges">
  ${PIXEL_R_SVG}
</svg>
`;

export const getLogoTypeSVG = (color: string) => `
<svg viewBox="0 0 184 32" xmlns="http://www.w3.org/2000/svg">
  <text x="0" y="25" fill="${color}" font-family="ui-monospace, SFMono-Regular, Menlo, monospace" font-size="25" font-weight="700" letter-spacing="-1.5">Term/UI</text>
</svg>
`;
