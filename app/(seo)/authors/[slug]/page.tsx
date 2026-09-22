import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";
import { notFound } from "next/navigation";

import { AuthorArticleCard } from "@/components/articles/author-article-card";
import { ArticleClosingCta } from "@/components/articles/closing-cta";
import { getAuthorBySlug, ARTICLE_AUTHORS } from "@/domains/articles/authors";
import { getArticleSummaries } from "@/domains/articles/content";
import { PersonJsonLd, BreadcrumbJsonLd } from "@/app/(seo)/_lib/json-ld";
import { createPageMetadata } from "@/app/(seo)/_lib/metadata";
import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

type Props = { params: Promise<{ slug: string }> };

export const dynamic = "force-static";
export const dynamicParams = false;
export const revalidate = false;

export const generateStaticParams = () =>
  ARTICLE_AUTHORS.map((author) => ({ slug: author.slug }));

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { slug } = await params;
  const author = getAuthorBySlug(slug);
  if (!author) return {};
  return createPageMetadata({
    description: author.bio,
    ogImage: `${SITE.URL}${ROUTES.AUTHORS}/${slug}/opengraph-image`,
    path: `${ROUTES.AUTHORS}/${slug}`,
    title: `${author.name} | Term/UI Author`,
  });
}

export default async function AuthorPage({ params }: Props) {
  const { slug } = await params;
  const author = getAuthorBySlug(slug);
  if (!author) notFound();
  const articles = (await getArticleSummaries()).filter(
    (article) => article.author === author.name,
  );
  const path = `${ROUTES.AUTHORS}/${author.slug}`;
  return (
    <div className="container-wrapper">
      <div className="container py-12 md:py-20">
        <BreadcrumbJsonLd
          items={[
            { name: "Home", path: ROUTES.HOME },
            { name: "Authors", path: ROUTES.AUTHORS },
            { name: author.name, path },
          ]}
        />
        <PersonJsonLd
          name={author.name}
          description={author.bio}
          url={`${SITE.URL}${path}`}
          image={`${SITE.URL}${author.avatar}`}
          sameAs={author.socials.map((social) => social.href)}
        />
        <header className="max-w-2xl">
          <Link href={ROUTES.AUTHORS} className="text-sm font-medium text-primary">
            ← All authors
          </Link>
          <Image
            src={author.avatar}
            alt={author.name}
            width={96}
            height={96}
            className="mt-6 size-24 rounded-full object-cover"
          />
          <h1 className="mt-4 text-4xl font-bold tracking-tight">{author.name}</h1>
          <p className="mt-2 text-lg text-muted-foreground">{author.role}</p>
          <p className="mt-5 leading-7 text-muted-foreground">{author.bio}</p>
          <div className="mt-5 flex gap-4 text-sm">
            {author.socials.map((social) => (
              <a
                key={social.href}
                href={social.href}
                target="_blank"
                rel="noreferrer"
                className="underline underline-offset-4"
              >
                {social.label}
              </a>
            ))}
          </div>
        </header>
        <section className="mt-12">
          <h2 className="text-2xl font-semibold">Articles by {author.name}</h2>
          <div className="mt-5 grid gap-5 md:grid-cols-2 lg:grid-cols-3">
            {articles.map((article) => (
              <AuthorArticleCard key={article.slug} article={article} />
            ))}
          </div>
        </section>
        <ArticleClosingCta />
      </div>
    </div>
  );
}
