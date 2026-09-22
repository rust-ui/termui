import { getStargazerCount } from "@/domains/site/github";

export const dynamic = "force-dynamic";

export async function GET() {
  try {
    const count = await getStargazerCount();

    return Response.json(
      { count },
      {
        headers: {
          "Cache-Control":
            "public, max-age=300, s-maxage=3600, stale-while-revalidate=86400",
        },
      },
    );
  } catch {
    return Response.json(
      { count: null },
      {
        status: 502,
        headers: { "Cache-Control": "no-store" },
      },
    );
  }
}
