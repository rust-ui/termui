import { collectMdxDemoReferences } from "./demo-mdx.mts";

export interface MdxSource {
  file: string;
  source: string;
}

export interface InteractiveDemo {
  name: string;
  title: string;
}

const INTERACTIVE_DEMO_SRC =
  /^\/demos\/interactive\.html\?demo=([a-z0-9]+(?:-[a-z0-9]+)*)$/;
const INTERACTIVE_SUFFIX = "-interactive";

export function collectInteractiveDemos(sources: readonly MdxSource[]) {
  const demos = new Map<string, InteractiveDemo>();

  for (const { file, source } of sources) {
    for (const reference of collectMdxDemoReferences(source)) {
      if (reference.tag !== "RustDemo" || !reference.src) continue;

      const match = INTERACTIVE_DEMO_SRC.exec(reference.src);
      if (!match) {
        throw new Error(
          `Unsupported interactive demo src "${reference.src}" in ${file}`
        );
      }

      const name = match[1];
      const label = (name.endsWith(INTERACTIVE_SUFFIX)
        ? name.slice(0, -INTERACTIVE_SUFFIX.length)
        : name
      )
        .split("-")
        .map((part) => part[0].toUpperCase() + part.slice(1))
        .join(" ");
      demos.set(name, { name, title: `Interactive Ratatui ${label}` });
    }
  }

  return [...demos.values()].sort((left, right) =>
    left.name.localeCompare(right.name)
  );
}

const TEMPLATE_MARKERS = {
  bin: "{{BIN_NAME}}",
  title: "{{PAGE_TITLE}}",
} as const;

function escapeHtml(value: string) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

export function renderInteractiveDemo(template: string, demo: InteractiveDemo) {
  const values: Record<string, string> = {
    [TEMPLATE_MARKERS.bin]: demo.name,
    [TEMPLATE_MARKERS.title]: escapeHtml(demo.title),
  };
  for (const marker of Object.keys(values)) {
    if (!template.includes(marker)) {
      throw new Error(`Missing template marker ${marker}`);
    }
  }
  const html = Object.entries(values).reduce(
    (result, [marker, value]) => result.replaceAll(marker, value),
    template
  );

  if (Object.keys(values).some((marker) => html.includes(marker))) {
    throw new Error(`Unresolved template marker in ${demo.name}`);
  }

  return html;
}
