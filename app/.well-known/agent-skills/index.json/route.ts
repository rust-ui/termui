import { ROUTES } from "@/constants/routes";
import { requestOrigin } from "@/lib/agent-discovery/request-origin";
import { termuiAgentSkillDigest } from "@/lib/agent-discovery/termui-agent-skill";

export const GET = (request: Request) => {
  const origin = requestOrigin(request);
  const base = origin.replace(/\/$/, "");

  return Response.json(
    {
      $schema: "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
      skills: [
        {
          description:
            "Install and use Term/UI terminal UI components via the public shadcn registry and documentation.",
          digest: termuiAgentSkillDigest(),
          name: "termui-registry",
          type: "skill-md",
          url: `${base}${ROUTES.AGENT_SKILLS_TERMUI_SKILL}`,
        },
      ],
    },
    { headers: { "Cache-Control": "public, max-age=3600" } }
  );
};
