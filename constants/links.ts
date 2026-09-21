export const GITHUB = {
  branch: "main",
  org: "rust-ui",
  repo: "termui",
} as const;

const GITHUB_URL = `https://github.com/${GITHUB.org}/${GITHUB.repo}`;

export const LINK = {
  GITHUB: GITHUB_URL,
  LICENSE: `${GITHUB_URL}/blob/${GITHUB.branch}/LICENSE`,
  PORTFOLIO: "https://rust-ui.com",
  SHADCN_MCP_DOCS: "https://ui.shadcn.com/docs/mcp",
} as const;
