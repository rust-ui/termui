"use client";

import { useCallback, useEffect, useState } from "react";

import { buildArticleShareLinks } from "@/domains/articles/share";

export type TocSection = { id: string; label: string };

const LABEL = "text-[12px] font-medium uppercase tracking-[0.04em] text-muted-foreground";
const SHARE_BTN =
  "flex size-9 items-center justify-center rounded-full border bg-background text-muted-foreground transition-colors hover:bg-muted";

const plainLabel = (label: string) => label.replace(/\*([^*]+)\*/g, "$1");

export function ArticleSidebar({
  sections,
  shareUrl,
  shareTitle,
}: {
  sections: TocSection[];
  shareUrl: string;
  shareTitle: string;
}) {
  const [activeId, setActiveId] = useState(sections[0]?.id ?? "");
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    const headings = sections
      .map((section) => document.getElementById(section.id))
      .filter((element): element is HTMLElement => element !== null);
    if (headings.length === 0) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
        if (visible[0]) setActiveId(visible[0].target.id);
      },
      { rootMargin: "-120px 0px -68% 0px", threshold: 0 },
    );
    headings.forEach((heading) => {
      observer.observe(heading);
    });
    return () => observer.disconnect();
  }, [sections]);

  const jump = useCallback((event: React.MouseEvent<HTMLAnchorElement>, id: string) => {
    event.preventDefault();
    const element = document.getElementById(id);
    if (!element) return;
    window.scrollTo({
      top: element.getBoundingClientRect().top + window.scrollY - 120,
      behavior: "smooth",
    });
    window.history.replaceState(null, "", `#${id}`);
    setActiveId(id);
  }, []);

  const copy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(shareUrl);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 2000);
    } catch {
      // Clipboard can be unavailable in an insecure context.
    }
  }, [shareUrl]);

  const openShare = useCallback(
    (href: string) =>
      window.open(href, "_blank", "noopener,noreferrer,width=600,height=540"),
    [],
  );

  return (
    <aside className="flex h-max flex-col gap-8 lg:sticky lg:top-[122px]">
      <div className="flex flex-col gap-3">
        <p className={LABEL}>Table of contents</p>
        <ul className="flex flex-col gap-1.5">
          {sections.map((section) => (
            <li key={section.id} className="min-w-0">
              <a
                href={`#${section.id}`}
                onClick={(event) => jump(event, section.id)}
                aria-current={section.id === activeId ? "true" : undefined}
                title={plainLabel(section.label)}
                className={`block truncate rounded-[7.78px] p-[5.83px] text-sm transition-colors ${section.id === activeId ? "bg-muted text-foreground" : "text-muted-foreground hover:bg-muted hover:text-foreground"}`}
              >
                {plainLabel(section.label)}
              </a>
            </li>
          ))}
        </ul>
      </div>
      <div className="flex flex-col gap-3 border-t pt-6">
        <p className={LABEL}>Share article</p>
        <div className="flex gap-2">
          <button
            type="button"
            onClick={copy}
            aria-label={copied ? "Copied" : "Copy to clipboard"}
            className={`${SHARE_BTN} ${copied ? "bg-green-100 text-green-700" : ""}`}
          >
            {copied ? "✓" : "↗"}
          </button>
          {buildArticleShareLinks(shareUrl, shareTitle).map((link) => (
            <button
              key={link.network}
              type="button"
              onClick={() => openShare(link.href)}
              aria-label={link.label}
              className={SHARE_BTN}
            >
              {link.network === "x" ? "𝕏" : link.network === "whatsapp" ? "◉" : "in"}
            </button>
          ))}
        </div>
      </div>
    </aside>
  );
}
