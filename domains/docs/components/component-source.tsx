import { CodeCollapsibleWrapper } from "@/components/shared/code-collapsible-wrapper";
import type {
  RatatuiComponentName,
  RatatuiComponentSource,
} from "@/domains/ratatui/config";
import { highlightCode } from "@/domains/docs/highlight-code";
import { readFileFromRoot } from "@/domains/ratatui/registry/read-file";
import { getDemoSource } from "@/domains/ratatui/registry/source";
import { cn } from "@/shared/lib/utils";

import { CopyButton } from "@/components/shared/copy-button";
import { getIconForLanguageExtension } from "@/components/shared/icons";

const ComponentCode = ({
  code,
  highlightedCode,
  language,
  title,
}: {
  code: string;
  highlightedCode: string;
  language: string;
  title: string | undefined;
}) => (
  <figure data-rehype-pretty-code-figure="" className="[&>pre]:max-h-96">
    {title && (
      <figcaption
        data-rehype-pretty-code-title=""
        className="flex items-center gap-2 text-code-foreground [&_svg]:size-4 [&_svg]:text-code-foreground [&_svg]:opacity-70"
        data-language={language}
      >
        {getIconForLanguageExtension(language)}
        {title}
      </figcaption>
    )}
    <CopyButton value={code} event="copy_primitive_code" />
    {/* eslint-disable-next-line react/no-danger */}
    <div dangerouslySetInnerHTML={{ __html: highlightedCode }} />
  </figure>
);

export const ComponentSource = async ({
  name,
  src,
  title,
  collapsible = true,
  className,
  language,
}: {
  name?: RatatuiComponentName;
  src?: RatatuiComponentSource;
  title?: string;
  collapsible?: boolean;
  className?: string;
  language?: string;
}) => {
  let code: string | null = null;

  if (name) {
    code = await getDemoSource(name);
  }

  if (src) {
    code = await readFileFromRoot(src);
  }

  if (!code) {
    return null;
  }

  const lang = language ?? title?.split(".").pop() ?? "rs";
  const highlightedCode = await highlightCode(code, lang);

  if (!collapsible) {
    return (
      <div className={cn("relative", className)}>
        <ComponentCode
          code={code}
          highlightedCode={highlightedCode}
          language={lang}
          title={title}
        />
      </div>
    );
  }

  return (
    <CodeCollapsibleWrapper
      className={className}
      navTriggerClassName={cn(!title && "top-3")}
    >
      <ComponentCode
        code={code}
        highlightedCode={highlightedCode}
        language={lang}
        title={title}
      />
    </CodeCollapsibleWrapper>
  );
};
