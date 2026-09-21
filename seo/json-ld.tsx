import { LINK } from "@/constants/links";
import { ROUTES } from "@/constants/routes";
import { SITE } from "@/constants/site";

const JsonLdScript = ({ data }: { data: Record<string, unknown> }) => (
  <script
    // eslint-disable-next-line react/no-danger
    dangerouslySetInnerHTML={{ __html: JSON.stringify(data) }}
    type="application/ld+json"
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
    logo: SITE.URL + "/logo.svg",
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
