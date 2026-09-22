export type ArticleAuthor = {
  slug: string;
  name: string;
  role: string;
  bio: string;
  avatar: string;
  socials: { label: string; href: string }[];
};

export const ARTICLE_AUTHORS: ArticleAuthor[] = [
  {
    slug: "max-wells",
    name: "Max Wells",
    role: "Term/UI editor",
    bio: "Max Wells edits Term/UI guides about building practical Rust terminal interfaces with Ratatui and copyable widgets.",
    avatar: "/articles/author-max-wells.webp",
    socials: [
      { label: "GitHub", href: "https://github.com/rust-ui/termui" },
      { label: "Term/UI", href: "https://termui.rustify.app" },
    ],
  },
];

export const getAuthorBySlug = (slug: string) =>
  ARTICLE_AUTHORS.find((author) => author.slug === slug);

export const getAuthorByName = (name: string) =>
  ARTICLE_AUTHORS.find((author) => author.name === name);
