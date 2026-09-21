import { notFound } from "next/navigation";

import { ChartGallery } from "@/components/charts/chart-gallery";
import { CHART_EXAMPLES, CHART_FAMILIES, getChartFamily } from "@/constants/charts";
import { ROUTES } from "@/constants/routes";
import { createPageMetadata } from "@/seo/metadata";

export const dynamic = "force-static";
export const dynamicParams = false;
export const revalidate = false;

export const generateStaticParams = () =>
  CHART_FAMILIES.map((family) => ({ type: family.type }));

export const generateMetadata = async ({
  params,
}: {
  params: Promise<{ type: string }>;
}) => {
  const { type } = await params;
  const family = getChartFamily(type);
  if (!family) notFound();

  return createPageMetadata({
    description: `${family.description} Explore Ratatui chart source and terminal previews.`,
    path: `${ROUTES.CHARTS}/${family.type}`,
    title: `Ratatui ${family.label}`,
  });
};

export default async function ChartFamilyPage({
  params,
}: {
  params: Promise<{ type: string }>;
}) {
  const { type } = await params;
  const family = getChartFamily(type);
  if (!family) notFound();

  return (
    <ChartGallery
      description={family.description}
      examples={CHART_EXAMPLES[family.type]}
      title={family.label}
    />
  );
}
