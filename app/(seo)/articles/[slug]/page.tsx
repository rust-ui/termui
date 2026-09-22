import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";
import { notFound } from "next/navigation";

import { ArticleSidebar, type TocSection } from "@/components/articles/article-sidebar";
import { ArticleClosingCta } from "@/components/articles/closing-cta";
import { ContentCardThumbnail } from "@/components/articles/content-card-thumbnail";
import { ARTICLE_CATEGORY_LABELS } from "@/domains/articles/categories";
import { getArticleBySlug, getArticleSummaries } from "@/domains/articles/content";
import { getAuthorByName } from "@/domains/articles/authors";
import { ArticleJsonLd, BreadcrumbJsonLd } from "@/app/(seo)/_lib/json-ld";
import { createPageMetadata } from "@/app/(seo)/_lib/metadata";
import { mdxComponents } from "@/mdx-components";
import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

type Props = { params: Promise<{ slug: string }> };

export const dynamicParams = false;

export async function generateStaticParams() {
  return (await getArticleSummaries()).map((article) => ({ slug: article.slug }));
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { slug } = await params;
  const article = await getArticleBySlug(slug);
  if (!article) return {};
  return createPageMetadata({
    description: article.description,
    ogImage: `${SITE.URL}${ROUTES.ARTICLES}/${slug}/opengraph-image`,
    ogImageAlt: article.title,
    ogType: "article",
    path: article.url,
    title: article.title,
  });
}

export default async function ArticlePage({ params }: Props) {
  const { slug } = await params;
  const article = await getArticleBySlug(slug);
  if (!article) notFound();
  const author = getAuthorByName(article.author);
  const authorPath = author ? `${ROUTES.AUTHORS}/${author.slug}` : ROUTES.AUTHORS;
  const path = article.url;
  const sections: TocSection[] = article.headings
    .filter((heading) => heading.level === 2)
    .map((heading) => ({ id: heading.id, label: heading.text }));
  const MdxContent = article.body;

  return (
    <div className="mx-auto max-w-[1200px] px-6 pb-24 pt-16 md:pt-24">
      <ArticleJsonLd
        title={article.title}
        description={article.description}
        url={`${SITE.URL}${path}`}
        image={`${SITE.URL}${article.image ?? "/og.png"}`}
        author={article.author}
        authorUrl={`${SITE.URL}${authorPath}`}
        publishDate={article.publishDate}
        lastUpdated={article.lastUpdated}
      />
      <BreadcrumbJsonLd
        items={[
          { name: "Home", path: ROUTES.HOME },
          { name: "Articles", path: ROUTES.ARTICLES },
          {
            name: ARTICLE_CATEGORY_LABELS[article.category],
            path: `${ROUTES.ARTICLES}/category/${article.category}`,
          },
          { name: article.title, path },
        ]}
      />
      <header className="grid gap-10 md:grid-cols-[minmax(0,1fr)_auto] md:items-start">
        <div className="flex flex-col gap-6">
          <nav className="flex items-center gap-2" aria-label="Breadcrumb">
            <Link
              href={ROUTES.ARTICLES}
              className="rounded-full border bg-black/[0.04] px-2.5 py-1 text-[11px] font-medium uppercase"
            >
              Articles
            </Link>
            <span className="text-sm text-muted-foreground">/</span>
            <Link
              href={`${ROUTES.ARTICLES}/category/${article.category}`}
              className="rounded-full border border-primary/30 bg-primary/10 px-2.5 py-1 text-[11px] font-medium uppercase text-primary"
            >
              {ARTICLE_CATEGORY_LABELS[article.category]}
            </Link>
          </nav>
          <h1 className="text-[clamp(38px,5vw,52px)] font-semibold leading-[1.1] tracking-[-0.042em]">
            {article.title}
          </h1>
          {author ? (
            <Link href={authorPath} className="flex w-fit items-center gap-3">
              <Image
                src={author.avatar}
                alt={author.name}
                width={44}
                height={44}
                className="size-11 rounded-full object-cover"
              />
              <span className="flex flex-col">
                <span className="text-sm font-medium">{article.author}</span>
                <span className="text-xs text-muted-foreground">{author.role}</span>
              </span>
            </Link>
          ) : (
            <span className="text-sm font-medium">{article.author}</span>
          )}
        </div>
        <div className="relative aspect-[1200/675] w-full overflow-hidden rounded-2xl bg-black md:w-[420px]">
          <ContentCardThumbnail
            title={article.shortTitleThumbnail}
            image={article.image}
            size="feature"
            priority
          />
        </div>
      </header>
      <div className="mt-16 grid gap-12 lg:grid-cols-[262px_minmax(0,1fr)]">
        <ArticleSidebar
          sections={sections}
          shareUrl={`${SITE.URL}${path}`}
          shareTitle={article.title}
        />
        <article className="min-w-0">
          <div className="mb-8 flex flex-wrap gap-3 text-sm text-muted-foreground">
            <time dateTime={article.publishDate}>{article.publishDate}</time>
            <span aria-hidden="true">·</span>
            <span>{article.readingTimeMinutes} min read</span>
            <span aria-hidden="true">·</span>
            <span>{article.description}</span>
          </div>
          <div className="prose prose-neutral dark:prose-invert max-w-none">
            <MdxContent components={mdxComponents} />
          </div>
          <div className="mt-16 rounded-2xl border bg-muted/30 p-6">
            <p className="font-medium">Keep building with Term/UI</p>
            <p className="mt-2 text-sm text-muted-foreground">
              Browse copyable Ratatui widgets and source examples for your next terminal
              interface.
            </p>
            <Link
              href={ROUTES.DOCS_WIDGETS}
              className="mt-4 inline-block text-sm font-medium underline underline-offset-4"
            >
              Browse widgets →
            </Link>
          </div>
        </article>
      </div>
      <ArticleClosingCta />
    </div>
  );
}
