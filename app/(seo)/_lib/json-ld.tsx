import { LINK } from "@/shared/config/links";
import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

const JsonLdScript = ({ data }: { data: Record<string, unknown> }) => (
  <script
    // eslint-disable-next-line react/no-danger
    dangerouslySetInnerHTML={{ __html: JSON.stringify(data) }}
    type="application/ld+json"
  />
);

export const ArticleJsonLd = ({
  title,
  description,
  url,
  image,
  author,
  authorUrl,
  publishDate,
  lastUpdated,
}: {
  title: string;
  description: string;
  url: string;
  image: string;
  author: string;
  authorUrl: string;
  publishDate: string;
  lastUpdated?: string;
}) => (
  <JsonLdScript
    data={{
      "@context": "https://schema.org",
      "@type": "Article",
      author: { "@type": "Person", name: author, url: authorUrl },
      dateModified: lastUpdated ?? publishDate,
      datePublished: publishDate,
      description,
      headline: title,
      image,
      mainEntityOfPage: url,
      publisher: { "@type": "Organization", name: SITE.NAME, url: SITE.URL },
      url,
    }}
  />
);

export const CollectionPageJsonLd = ({
  name,
  description,
  url,
  items,
}: {
  name: string;
  description: string;
  url: string;
  items: { name: string; url: string }[];
}) => (
  <JsonLdScript
    data={{
      "@context": "https://schema.org",
      "@type": "CollectionPage",
      description,
      mainEntity: {
        "@type": "ItemList",
        itemListElement: items.map((item, index) => ({
          "@type": "ListItem",
          item: { "@type": "Article", name: item.name, url: item.url },
          position: index + 1,
        })),
      },
      name,
      url,
    }}
  />
);

export const PersonJsonLd = ({
  name,
  description,
  url,
  image,
  sameAs,
}: {
  name: string;
  description: string;
  url: string;
  image: string;
  sameAs: string[];
}) => (
  <JsonLdScript
    data={{
      "@context": "https://schema.org",
      "@type": "Person",
      description,
      image,
      name,
      sameAs,
      url,
    }}
  />
);

export const WebsiteJsonLd = () => {
  const data = {
    "@context": "https://schema.org",
    "@type": "WebSite",
    description: SITE.DESCRIPTION.LONG,
    inLanguage: "en-US",
    name: SITE.NAME,
    url: SITE.URL,
  };
  return <JsonLdScript data={data} />;
};

export const SoftwareSourceCodeJsonLd = () => {
  const data = {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    applicationCategory: "DeveloperApplication",
    author: {
      "@type": "Organization",
      name: SITE.NAME,
      url: SITE.URL,
    },
    codeRepository: LINK.GITHUB,
    description: SITE.DESCRIPTION.LONG,
    isAccessibleForFree: true,
    license: LINK.LICENSE,
    maintainer: {
      "@type": "Organization",
      name: SITE.NAME,
      url: SITE.URL,
    },
    name: SITE.NAME,
    programmingLanguage: "Rust",
    url: SITE.URL,
  };
  return <JsonLdScript data={data} />;
};

export const OrganizationJsonLd = () => {
  const data = {
    "@context": "https://schema.org",
    "@type": "Organization",
    logo: `${SITE.URL}/logo.svg`,
    name: SITE.NAME,
    sameAs: [LINK.GITHUB],
    url: SITE.URL,
  };
  return <JsonLdScript data={data} />;
};

export const BreadcrumbJsonLd = ({
  items,
}: {
  items: { name: string; path: string }[];
}) => {
  const data = {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: items.map((item, index) => {
      const pathname = item.path.startsWith(ROUTES.HOME)
        ? item.path
        : `${ROUTES.HOME}${item.path}`;
      return {
        "@type": "ListItem",
        item: `${SITE.URL}${pathname}`,
        name: item.name,
        position: index + 1,
      };
    }),
  };
  return <JsonLdScript data={data} />;
};

export const JsonLdScripts = () => (
  <>
    <WebsiteJsonLd />
    <SoftwareSourceCodeJsonLd />
    <OrganizationJsonLd />
  </>
);
