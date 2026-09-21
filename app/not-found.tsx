import type { Metadata } from "next";

import { NotFound as PageNotFound } from "@/domains/not-found/components/not-found";

export const metadata: Metadata = {
  title: "Page Not Found",
};

const NotFound = () => <PageNotFound />;

export default NotFound;
