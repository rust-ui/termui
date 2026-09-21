import { llms } from "fumadocs-core/source";

import { ROUTES } from "@/constants/routes";
import { SITE } from "@/constants/site";
import { requestOrigin } from "@/lib/agent-discovery/request-origin";
import { source } from "@/lib/source";

export const revalidate = false;

const documentationIndex = async () =>
  (await llms(source)
    .index()
  )
    .replace(/^#\s+(.+)$/m, "## $1")
    .replaceAll(
      /\]\((\/docs(?:\/[^)#\s]+)?)(#[^)]+)?\)/g,
      (_, pathname, hash = "") => `](${pathname}.md${hash})`
    )
    .trim();

const docsIndex = async (origin: string) => {
  const base = origin.replace(/\/$/, "");

  return `# ${SITE.NAME}

> ${SITE.DESCRIPTION.LONG} Use this index to find the widget catalog, copy instructions, Rust source, and terminal demos.

${await documentationIndex()}

## Machine-readable Resources

- [Full documentation](${base}${ROUTES.LLMS_FULL})
- [Homepage markdown](${base}${ROUTES.LLMS_MD}/content.md)
- [OpenAPI description](${base}${ROUTES.OPENAPI})
- [API catalog](${base}${ROUTES.API_CATALOG})
- [Agent skill](${base}${ROUTES.AGENT_SKILLS_TERMUI_SKILL})
`;
};

export const GET = async (request: Request) =>
  new Response(await docsIndex(requestOrigin(request)), {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
    },
  });
