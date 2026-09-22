import {
  defineConfig,
  defineDocs,
  frontmatterSchema,
  metaSchema,
} from "fumadocs-mdx/config";
import { rehypePrettyCode } from "rehype-pretty-code";
import { z } from "zod";

import { DOCS_DIR } from "@/domains/docs/docs";
import { transformers } from "@/domains/docs/highlight-code";
import { ARTICLE_CATEGORIES } from "@/domains/articles/categories";

const isoDate = z
  .string()
  .regex(/^\d{4}-\d{2}-\d{2}$/)
  .refine(
    (value) => !Number.isNaN(Date.parse(`${value}T00:00:00Z`)),
    "Invalid calendar date",
  );

const articleSchema = z.object({
  title: z.string().min(1).max(120),
  description: z.string().min(1).max(240),
  keywords: z.array(z.string().min(1).max(80)).default([]),
  category: z.enum(ARTICLE_CATEGORIES),
  order: z.number().int().positive(),
  publish_date: isoDate,
  last_updated: isoDate.optional(),
  author: z.string().min(1).max(80),
  author_image: z.string().startsWith("/"),
  short_title_thumbnail: z.string().min(1).max(48),
  image: z.string().startsWith("/").optional(),
  image_alt: z.string().max(160).optional(),
});

export default defineConfig({
  mdxOptions: {
    rehypePlugins: (plugins) => {
      plugins.shift();
      plugins.push([
        rehypePrettyCode,
        {
          theme: {
            dark: "github-dark",
            light: "github-light-default",
          },
          transformers,
        },
      ]);

      return plugins;
    },
  },
});

export const docs = defineDocs({
  dir: DOCS_DIR,
  docs: {
    postprocess: {
      includeProcessedMarkdown: true,
    },
    schema: frontmatterSchema,
  },
  meta: {
    schema: metaSchema,
  },
});

export const articles = defineDocs({
  dir: "content/articles",
  docs: {
    postprocess: {
      includeProcessedMarkdown: true,
    },
    schema: articleSchema,
  },
});
