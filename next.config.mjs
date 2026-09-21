import { createMDX } from "fumadocs-mdx/next";
import { createJiti } from "jiti";

const jiti = createJiti(import.meta.url);
const { LINK } = await jiti.import("./constants/links");
const { ROUTES } = await jiti.import("./constants/routes");
const { RATATUI_COMPONENTS } = await jiti.import("./constants/ratatui");

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  devIndicators: false,
  headers() {
    const link = [
      '</.well-known/api-catalog>; rel="api-catalog"',
      '</openapi.json>; rel="service-desc"',
      '</docs>; rel="service-doc"',
      `<${LINK.SHADCN_MCP_DOCS}>; rel="service-doc"; title="shadcn MCP server"`,
      '</.well-known/agent-skills/index.json>; rel="describedby"',
    ].join(", ");

    return [{ headers: [{ key: "Link", value: link }], source: ROUTES.HOME }];
  },
  images: {
    remotePatterns: [
      {
        hostname: "avatars.githubusercontent.com",
        protocol: "https",
      },
      {
        hostname: "images.unsplash.com",
        protocol: "https",
      },
    ],
  },
  redirects() {
    return [
      {
        destination: `${ROUTES.DOCS}.md`,
        permanent: true,
        source: `${ROUTES.DOCS}.mdx`,
      },
      {
        destination: `${ROUTES.DOCS}/:path*.md`,
        permanent: true,
        source: `${ROUTES.DOCS}/:path*.mdx`,
      },
      ...[
        ROUTES.DOCS_CHARTS,
        ROUTES.DOCS_TEMPLATES,
        ROUTES.DOCS_THEMES,
        ROUTES.DOCS_THEMING,
      ].map((source) => ({
        destination: ROUTES.DOCS,
        permanent: true,
        source: `${source}/:path*`,
      })),
      {
        destination: ROUTES.DOCS_WIDGETS,
        permanent: true,
        source: ROUTES.WIDGETS,
      },
      {
        destination: `${ROUTES.DOCS_WIDGETS}/:path*`,
        permanent: true,
        source: `${ROUTES.DOCS_COMPONENTS_LEGACY}/:path*`,
      },
      ...RATATUI_COMPONENTS.map(({ name }) => ({
        destination: `${ROUTES.DOCS_WIDGETS}/${name}`,
        permanent: true,
        source: `${ROUTES.WIDGETS}/${name}`,
      })),
      {
        destination: ROUTES.DOCS_WIDGETS,
        permanent: true,
        source: ROUTES.LEGACY_COMPONENTS,
      },
      ...RATATUI_COMPONENTS.map(({ name }) => ({
        destination: `${ROUTES.DOCS_WIDGETS}/${name}`,
        permanent: true,
        source: `${ROUTES.LEGACY_COMPONENTS}/${name}`,
      })),
    ];
  },
  rewrites() {
    return {
      // Legacy flat registry URLs → canonical nested paths under public/r/{ink,opentui}/.
      // registry.json stays at /r/registry.json (served as a static file before these run).
      afterFiles: [
        {
          destination: "/r/opentui/:slug.json",
          source: "/r/opentui-:slug.json",
        },
        {
          destination: "/r/ink/:slug.json",
          source: "/r/:slug.json",
        },
      ],
    };
  },
};

const withMDX = createMDX({});

export default withMDX(nextConfig);
