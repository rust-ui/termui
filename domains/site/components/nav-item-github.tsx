import { getStargazerCount } from "@/domains/site/github";

import { GitHubStars } from "./github-stars";

export const NavItemGithub = async () => {
  const stargazersCount = await getStargazerCount();

  return <GitHubStars stargazersCount={stargazersCount} />;
};
