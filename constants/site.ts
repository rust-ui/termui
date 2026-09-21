export const FALLBACK_SITE_ORIGIN = "https://rust-ui.com" as const;

const getBaseUrl = () => {
  if (process.env.NODE_ENV !== "production") {
    return "http://localhost:3000";
  }
  return process.env.SITE_URL ?? FALLBACK_SITE_ORIGIN;
};

const baseUrl = getBaseUrl();

export const SITE = {
  AUTHOR: {
    NAME: "Rust/UI contributors",
    TWITTER: "@termui",
  },
  DESCRIPTION: {
    LONG: "Term/UI is a collection of accessible, customizable terminal UI components in the Rust/UI ecosystem. Its previews are generated from Rust.",
    SHORT: "Beautiful terminal UIs, made simple",
  },
  KEYWORDS: [
    "Term/UI",
    "Rust/UI",
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
  NAME: "Term/UI",
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
