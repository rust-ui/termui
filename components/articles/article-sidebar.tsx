"use client";

import { useCallback, useEffect, useState } from "react";

import { buildArticleShareLinks } from "@/domains/articles/share";

export type TocSection = { id: string; label: string };

const LABEL = "text-[12px] font-medium uppercase tracking-[0.04em] text-muted-foreground";
const SHARE_BTN =
  "flex size-9 items-center justify-center rounded-full border bg-background text-muted-foreground transition-colors hover:bg-muted";

function IconLink() {
  return (
    <svg viewBox="0 0 24 24" fill="none" className="size-[19px]" aria-hidden>
      <path
        d="M9 15 15 9M10.5 6.5l.8-.9a4 4 0 0 1 5.7 5.7l-.9.8M13.5 17.5l-.8.9a4 4 0 0 1-5.7-5.7l.9-.8"
        stroke="currentColor"
        strokeWidth="1.7"
        strokeLinecap="round"
      />
    </svg>
  );
}

function IconCheck() {
  return (
    <svg viewBox="0 0 24 24" fill="none" className="size-[19px]" aria-hidden>
      <path
        d="m5 12.5 4.5 4.5L19 7"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function IconX() {
  return (
    <svg viewBox="0 0 18 18" fill="#0f0f10" className="size-[16px]" aria-hidden>
      <path d="M13.053 2.625h2.16l-4.72 5.4 5.554 7.35h-4.35l-3.406-4.457-3.9 4.457H2.23l5.05-5.775L1.96 2.625h4.46l3.08 4.073 3.553-4.073Zm-.758 11.454h1.197L5.76 3.83H4.476l7.82 10.249Z" />
    </svg>
  );
}

function IconWhatsApp() {
  return (
    <svg viewBox="0 0 24 24" fill="#25D366" className="size-[19px]" aria-hidden>
      <path d="M12.04 2c-5.46 0-9.91 4.45-9.91 9.91 0 1.75.46 3.45 1.32 4.95L2 22l5.25-1.38c1.45.79 3.08 1.21 4.79 1.21 5.46 0 9.91-4.45 9.91-9.91 0-2.65-1.03-5.14-2.9-7.01A9.82 9.82 0 0 0 12.04 2Zm0 1.67c2.2 0 4.27.86 5.82 2.42a8.18 8.18 0 0 1 2.41 5.82c0 4.54-3.69 8.23-8.24 8.23-1.48 0-2.93-.4-4.19-1.15l-.3-.18-3.12.82.83-3.04-.2-.31a8.19 8.19 0 0 1-1.26-4.37c0-4.54 3.7-8.23 8.24-8.23Zm4.52 9.72c-.25-.12-1.47-.72-1.69-.8-.23-.09-.39-.13-.56.12-.16.25-.64.8-.78.97-.14.16-.29.18-.54.06-.25-.12-1.05-.39-1.99-1.23-.74-.66-1.23-1.47-1.38-1.72-.14-.25-.02-.38.11-.51.11-.11.25-.29.37-.43.12-.14.16-.25.25-.41.08-.16.04-.31-.02-.43-.06-.12-.56-1.34-.76-1.84-.2-.48-.4-.42-.56-.42-.14 0-.31-.02-.47-.02s-.43.06-.66.31c-.23.25-.86.85-.86 2.07 0 1.22.89 2.4 1.01 2.56.12.16 1.75 2.67 4.25 3.74.59.26 1.06.41 1.42.52.6.19 1.14.16 1.57.1.48-.07 1.47-.6 1.68-1.18.21-.58.21-1.07.14-1.18-.06-.11-.22-.17-.47-.29Z" />
    </svg>
  );
}

function IconLinkedIn() {
  return (
    <svg viewBox="0 0 24 24" fill="#0A66C2" className="size-[18px]" aria-hidden>
      <path d="M6.94 5a2 2 0 1 1-4-.002 2 2 0 0 1 4 .002ZM7 8.48H3V21h4V8.48Zm6.32 0H9.34V21h3.94v-6.57c0-3.66 4.77-4 4.77 0V21H22v-7.93c0-6.17-7.06-5.94-8.68-2.91V8.48Z" />
    </svg>
  );
}

const SHARE_TARGET_UI = {
  x: <IconX />,
  whatsapp: <IconWhatsApp />,
  linkedin: <IconLinkedIn />,
} as const;

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
            {copied ? <IconCheck /> : <IconLink />}
          </button>
          {buildArticleShareLinks(shareUrl, shareTitle).map((link) => (
            <button
              key={link.network}
              type="button"
              onClick={() => openShare(link.href)}
              aria-label={link.label}
              className={SHARE_BTN}
            >
              {SHARE_TARGET_UI[link.network]}
            </button>
          ))}
        </div>
      </div>
    </aside>
  );
}
