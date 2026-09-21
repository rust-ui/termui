import { TERMUI_AGENT_SKILL_MD } from "@/domains/discovery/agent-discovery/termui-agent-skill";

export const GET = () =>
  new Response(TERMUI_AGENT_SKILL_MD, {
    headers: {
      "Cache-Control": "public, max-age=3600",
      "Content-Type": "text/markdown; charset=utf-8",
    },
  });
