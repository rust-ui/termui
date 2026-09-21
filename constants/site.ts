export const FALLBACK_SITE_ORIGIN = "https://termui.rustify.app" as const;

const getBaseUrl = () => {
  if (process.env.NODE_ENV !== "production") {
    return "http://localhost:3000";
  }
  return process.env.SITE_URL ?? FALLBACK_SITE_ORIGIN;
};

const baseUrl = getBaseUrl();

export const SITE = {
  AUTHOR: {
    NAME: "TERMUI contributors",
    TWITTER: "@termui",
  },
  DESCRIPTION: {
    LONG: "A collection of beautifully designed, accessible, and customizable terminal UI components, rendered from Rust.",
    SHORT: "Beautiful terminal UIs, made simple",
  },
  KEYWORDS: [
    "terminal",
    "rust",
    "ink",
    "react terminal",
    "next.js terminal",
    "shadcn",
    "component registry",
    "react components",
    "next.js",
    "npx shadcn add",
  ] as const,
  NAME: "termui",
  OG_IMAGE: `${baseUrl}/og.png`,
  REGISTRY: "@termui",
  URL: baseUrl,
};

export const META_THEME_COLORS = {
  dark: "#0a0a0a",
  light: "#ffffff",
};

export const UTM_PARAMS = {
  utm_source: new URL(baseUrl).hostname,
};
