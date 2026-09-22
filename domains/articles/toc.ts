export const tocTitleToText = (value: unknown): string => {
  if (typeof value === "string" || typeof value === "number") return String(value);
  if (Array.isArray(value)) return value.map(tocTitleToText).join("");
  if (!value || typeof value !== "object") return "";

  const node = value as {
    children?: unknown;
    props?: { children?: unknown };
    value?: unknown;
  };
  return tocTitleToText(node.props?.children ?? node.children ?? node.value);
};
