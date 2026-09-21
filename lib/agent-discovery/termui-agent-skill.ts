import { createHash } from "node:crypto";

import { LINK } from "@/constants/links";
import { ROUTES } from "@/constants/routes";
import { SITE } from "@/constants/site";

export const TERMUI_AGENT_SKILL_MD = `# ${SITE.NAME} — terminal UI component registry

## Summary

Help users add **${SITE.NAME}** components via the shadcn CLI against the public registry. Term/UI is part of the Rust/UI ecosystem.

## Registry

- Registry JSON: \`/r/registry.json\` (shadcn schema)
- Docs: ${ROUTES.DOCS_REGISTRY}
- shadcn MCP docs: ${LINK.SHADCN_MCP_DOCS}

## Install (shadcn)

\`\`\`bash
npx shadcn@latest add ${SITE.URL}/r/badge.json
\`\`\`

Prefer following the on-site installation guide: ${ROUTES.DOCS_INSTALLATION}

## When answering

- Prefer linking to \`${ROUTES.DOCS}\` sections over guessing props.
- Components live under Ink and OpenTUI namespaces in the docs.
`;

export const termuiAgentSkillDigest = (): string => {
  const hex = createHash("sha256")
    .update(TERMUI_AGENT_SKILL_MD, "utf-8")
    .digest("hex");

  return `sha256:${hex}`;
};
