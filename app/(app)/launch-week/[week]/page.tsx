import { notFound } from "next/navigation";

import { DirectionalTransition } from "@/components/shared/directional-transition";
import { LaunchWeek } from "@/domains/launch-week/components/launch-week";
import { ROUTES } from "@/shared/config/routes";
import {
  formatLaunchWeekRange,
  getLaunchWeek,
  getLaunchWeekHref,
  getLaunchWeeks,
} from "@/domains/launch-week/data";
import { BreadcrumbJsonLd } from "@/domains/site/seo/json-ld";
import { createPageMetadata } from "@/domains/site/seo/metadata";

export const dynamic = "force-static";
export const dynamicParams = false;
export const revalidate = false;

export const generateStaticParams = () =>
  getLaunchWeeks().map((week) => ({ week: week.slug }));

export const generateMetadata = async ({
  params,
}: {
  params: Promise<{ week: string }>;
}) => {
  const { week: slug } = await params;
  const week = getLaunchWeek(slug);

  if (!week) {
    notFound();
  }

  return createPageMetadata({
    description: week.description,
    noIndex: true,
    path: getLaunchWeekHref(week),
    title: `${week.title}: ${formatLaunchWeekRange(week)}`,
  });
};

export default async function LaunchWeekPage({
  params,
}: {
  params: Promise<{ week: string }>;
}) {
  const { week: slug } = await params;
  const week = getLaunchWeek(slug);

  if (!week) {
    notFound();
  }

  return (
    <>
      <BreadcrumbJsonLd
        items={[
          { name: "Home", path: ROUTES.HOME },
          { name: "Launch Weeks", path: ROUTES.LAUNCH_WEEK },
          { name: week.title, path: getLaunchWeekHref(week) },
        ]}
      />
      <DirectionalTransition>
        <LaunchWeek week={week} />
      </DirectionalTransition>
    </>
  );
}
