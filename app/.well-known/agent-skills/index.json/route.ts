import { ROUTES } from "@/shared/config/routes";
import { requestOrigin } from "@/domains/discovery/agent-discovery/request-origin";
import { termuiAgentSkillDigest } from "@/domains/discovery/agent-discovery/termui-agent-skill";

export const GET = (request: Request) => {
  const origin = requestOrigin(request);
  const base = origin.replace(/\/$/, "");

  return Response.json(
    {
      $schema: "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
      skills: [
        {
          description:
            "Find and copy Term/UI Ratatui widgets, Rust source, and composition guidance.",
          digest: termuiAgentSkillDigest(),
          name: "termui-ratatui-widgets",
          type: "skill-md",
          url: `${base}${ROUTES.AGENT_SKILLS_TERMUI_SKILL}`,
        },
      ],
    },
    { headers: { "Cache-Control": "public, max-age=3600" } },
  );
};
