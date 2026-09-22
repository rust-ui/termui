import { unstable_cache } from "next/cache";

import { GITHUB } from "@/shared/config/links";

interface Stargazer {
  login: string;
  avatar_url: string;
}

export const getStargazers = unstable_cache(
  async (): Promise<Stargazer[]> => {
    try {
      const pages: Stargazer[] = [];
      let page = 1;

      while (page <= 5) {
        const response = await fetch(
          `https://api.github.com/repos/${GITHUB.org}/${GITHUB.repo}/stargazers?per_page=100&page=${page}`,
          {
            headers: {
              Accept: "application/vnd.github+json",
              "X-GitHub-Api-Version": "2022-11-28",
            },
          },
        );

        if (!response.ok) {
          break;
        }

        const data: Stargazer[] = await response.json();
        if (data.length === 0) {
          break;
        }

        pages.push(...data);
        if (data.length < 100) {
          break;
        }
        page += 1;
      }

      return pages.filter((s) => s.login !== GITHUB.org);
    } catch {
      return [];
    }
  },
  ["github-stargazers"],
  { revalidate: 86_400 },
);

export const getStargazerCount = async (): Promise<number> => {
  const response = await fetch(
    `https://api.github.com/repos/${GITHUB.org}/${GITHUB.repo}`,
    {
      headers: {
        Accept: "application/vnd.github+json",
        "User-Agent": "termui.rustify.app",
        "X-GitHub-Api-Version": "2022-11-28",
      },
      next: { revalidate: 3_600 },
    },
  );

  if (!response.ok) {
    throw new Error(`GitHub API returned ${response.status}`);
  }

  const json = (await response.json()) as { stargazers_count?: number };
  const count = json.stargazers_count;

  if (typeof count !== "number" || !Number.isInteger(count) || count < 0) {
    throw new Error("GitHub API returned an invalid star count");
  }

  return count;
};
