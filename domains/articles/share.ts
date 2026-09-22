export const buildArticleShareLinks = (articleUrl: string, articleTitle: string) => {
  const url = encodeURIComponent(articleUrl);
  const title = encodeURIComponent(articleTitle);
  return [
    {
      network: "x",
      label: "Share on X",
      href: `https://twitter.com/intent/tweet?url=${url}&text=${title}`,
    },
    {
      network: "whatsapp",
      label: "Share on WhatsApp",
      href: `https://wa.me/?text=${title}%20${url}`,
    },
    {
      network: "linkedin",
      label: "Share on LinkedIn",
      href: `https://www.linkedin.com/sharing/share-offsite/?url=${url}`,
    },
  ] as const;
};
