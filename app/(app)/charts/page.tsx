import { redirect } from "next/navigation";

import { CHART_FAMILIES } from "@/domains/charts/config";
import { ROUTES } from "@/shared/config/routes";

export default function ChartsIndexPage() {
  redirect(`${ROUTES.CHARTS}/${CHART_FAMILIES[0].type}`);
}
