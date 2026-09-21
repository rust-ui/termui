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
    NAME: "Term/UI contributors",
    TWITTER: "@termui",
  },
  DESCRIPTION: {
    LONG: "Term/UI is a collection of copyable, customizable Ratatui widgets for Rust terminal applications. Browse Rust source and Rust-rendered previews, then adapt each component to your app.",
    SHORT: "Copyable Ratatui widgets for Rust terminal apps",
  },
  NAME: "Term/UI",
  OG_IMAGE: `${baseUrl}/og.png`,
  URL: baseUrl,
};

export const META_THEME_COLORS = {
  dark: "#0a0a0a",
  light: "#ffffff",
};

export const UTM_PARAMS = {
  utm_source: new URL(baseUrl).hostname,
};
