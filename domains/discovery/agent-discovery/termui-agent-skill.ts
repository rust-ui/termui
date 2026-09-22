import { createHash } from "node:crypto";

import { ROUTES } from "@/shared/config/routes";
import { SITE } from "@/shared/config/site";

export const TERMUI_AGENT_SKILL_MD = `# ${SITE.NAME} — Ratatui widgets

## Summary

Help users find and copy **${SITE.NAME}** Rust widgets into Ratatui terminal applications. Term/UI is an independent product; its widgets are source modules, not a published Cargo crate.

## Documentation

- Widget catalog: \`${ROUTES.DOCS_WIDGETS}\`
- Installation: \`${ROUTES.DOCS_INSTALLATION}\`

## Use the widgets

Copy source from the relevant widget page. Include any sibling modules it imports and adapt the \`crate::\` paths to the host application. The application owns state, event handling, and its terminal event loop.

## When answering

- Prefer linking to the specific widget and article pages over guessing API details.
- Do not describe Term/UI as a Cargo dependency or a shadcn registry.
`;

export const termuiAgentSkillDigest = (): string => {
  const hex = createHash("sha256").update(TERMUI_AGENT_SKILL_MD, "utf-8").digest("hex");

  return `sha256:${hex}`;
};
