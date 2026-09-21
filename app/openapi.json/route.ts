import { buildOpenApiDocument } from "@/domains/discovery/agent-discovery/openapi-document";
import { requestOrigin } from "@/domains/discovery/agent-discovery/request-origin";

export const GET = (request: Request) => {
  const origin = requestOrigin(request);
  const doc = buildOpenApiDocument(origin);

  return Response.json(doc, {
    headers: { "Cache-Control": "public, max-age=3600" },
  });
};
