import { AuthorCard } from "@/components/articles/author-card";
import { ArticleClosingCta } from "@/components/articles/closing-cta";
import { ARTICLE_AUTHORS } from "@/domains/articles/authors";
import { getArticleSummaries } from "@/domains/articles/content";
import { createPageMetadata } from "@/app/(seo)/_lib/metadata";
import { ROUTES } from "@/shared/config/routes";

export const metadata = createPageMetadata({
  description:
    "Meet the Term/UI contributors writing practical Ratatui and Rust terminal UI guides.",
  path: ROUTES.AUTHORS,
  title: "Term/UI Authors",
});

export const dynamic = "force-static";
export const revalidate = false;

export default async function AuthorsPage() {
  const articles = await getArticleSummaries();
  return (
    <div className="container-wrapper">
      <div className="container py-12 md:py-20">
        <header className="max-w-2xl">
          <p className="text-sm font-medium text-primary">Term/UI editorial team</p>
          <h1 className="mt-3 text-4xl font-bold tracking-tight">Authors</h1>
          <p className="mt-5 text-lg leading-8 text-muted-foreground">
            Practical Rust terminal UI writing based on current Ratatui APIs, tested
            examples, and copyable widget source.
          </p>
        </header>
        <div className="mt-10 grid gap-5 md:grid-cols-2 lg:grid-cols-3">
          {ARTICLE_AUTHORS.map((author) => (
            <AuthorCard
              key={author.slug}
              slug={author.slug}
              name={author.name}
              role={author.role}
              avatar={author.avatar}
              articleCount={
                articles.filter((article) => article.author === author.name).length
              }
            />
          ))}
        </div>
        <ArticleClosingCta />
      </div>
    </div>
  );
}
