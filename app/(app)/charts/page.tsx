import { redirect } from "next/navigation";

import { CHART_FAMILIES } from "@/constants/charts";
import { ROUTES } from "@/constants/routes";

export default function ChartsIndexPage() {
  redirect(`${ROUTES.CHARTS}/${CHART_FAMILIES[0].type}`);
}
