import { cn } from "@/lib/utils";

const LOGO_FRAME_PATH = "M155 45c0 5.523-4.477 10-10 10h-30c-5.523 0-10 4.477-10 10v30c0 5.523-4.477 10-10 10H65c-5.523 0-10 4.477-10 10v225c0 5.523 4.477 10 10 10h30c5.523 0 10 4.477 10 10v30c0 5.523 4.477 10 10 10h30c5.523 0 10 4.477 10 10v35c0 5.523-4.477 10-10 10h-35c-5.523 0-10-4.477-10-10v-30c0-5.523-4.477-10-10-10H60c-5.523 0-10-4.477-10-10v-30c0-5.523-4.477-10-10-10H10c-5.523 0-10-4.477-10-10V110c0-5.523 4.477-10 10-10h30c5.523 0 10-4.477 10-10V60c0-5.523 4.477-10 10-10h30c5.523 0 10-4.477 10-10V10c0-5.523 4.477-10 10-10h35c5.523 0 10 4.477 10 10zm250-5c0 5.523 4.477 10 10 10h30c5.523 0 10 4.477 10 10v30c0 5.523 4.477 10 10 10h30c5.523 0 10 4.477 10 10v235c0 5.523-4.477 10-10 10h-30c-5.523 0-10 4.477-10 10v30c0 5.523-4.477 10-10 10h-30c-5.523 0-10 4.477-10 10v30c0 5.523-4.477 10-10 10h-35c-5.523 0-10-4.477-10-10v-35c0-5.523 4.477-10 10-10h30c5.523 0 10-4.477 10-10v-30c0-5.523 4.477-10 10-10h30c5.523 0 10-4.477 10-10V115c0-5.523-4.477-10-10-10h-30c-5.523 0-10-4.477-10-10V65c0-5.523-4.477-10-10-10h-30c-5.523 0-10-4.477-10-10V10c0-5.523 4.477-10 10-10h35c5.523 0 10 4.477 10 10z";
const LOGO_R_PATH = "M205 50H305Q355 50 355 100V200Q355 250 305 250H255L355 350V400H305L205 300V400H155V50H205Z M215 100H295Q305 100 305 110V190Q305 200 295 200H215Q205 200 205 190V110Q205 100 215 100Z";
const LOGO_R_TRANSFORM = "translate(25.25 0) scale(.9 1)";

export const LogoMark = ({ className, ...props }: React.ComponentProps<"svg">) => (
  <svg
    aria-hidden="true"
    viewBox="0 0 505 455"
    fill="currentColor"
    xmlns="http://www.w3.org/2000/svg"
    className={cn("size-4 text-foreground", className)}
    {...props}
  >
    <path d={LOGO_FRAME_PATH} />
    <path d={LOGO_R_PATH} fillRule="evenodd" transform={LOGO_R_TRANSFORM} />
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
<svg viewBox="0 0 505 455" fill="${color}" xmlns="http://www.w3.org/2000/svg">
  <path d="${LOGO_FRAME_PATH}" />
  <path d="${LOGO_R_PATH}" fill-rule="evenodd" transform="${LOGO_R_TRANSFORM}" />
</svg>
`;

export const getLogoTypeSVG = (color: string) => `
<svg viewBox="0 0 184 32" xmlns="http://www.w3.org/2000/svg">
  <text x="0" y="25" fill="${color}" font-family="ui-monospace, SFMono-Regular, Menlo, monospace" font-size="25" font-weight="700" letter-spacing="-1.5">Term/UI</text>
</svg>
`;
