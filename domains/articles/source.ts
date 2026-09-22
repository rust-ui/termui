import { loader } from "fumadocs-core/source";

import { articles } from "@/.source/server";

export const articleSource = loader({
  baseUrl: "/articles",
  source: articles.toFumadocsSource(),
});
