export const GITHUB = {
  branch: "main",
  org: "rust-ui",
  repo: "termui",
} as const;

const GITHUB_URL = `https://github.com/${GITHUB.org}/${GITHUB.repo}`;

export const LINK = {
  GITHUB: GITHUB_URL,
  LICENSE: `${GITHUB_URL}/blob/${GITHUB.branch}/LICENSE`,
  PORTFOLIO: "https://termui.rs",
} as const;
