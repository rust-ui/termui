import Link from "next/link";

import { ROUTES } from "@/shared/config/routes";

export function ArticleClosingCta() {
  return (
    <section className="mt-28 flex flex-col gap-10">
      <div className="flex flex-col gap-5 text-center">
        <h2 className="mx-auto max-w-[20ch] text-4xl font-semibold leading-tight tracking-tight md:text-5xl">
          Build your next terminal UI with Ratatui.
        </h2>
        <div>
          <Link
            href={ROUTES.DOCS_WIDGETS}
            className="inline-flex rounded-full bg-primary px-5 py-2.5 text-sm font-semibold text-primary-foreground transition-transform hover:-translate-y-0.5"
          >
            Browse copyable widgets ↗
          </Link>
        </div>
      </div>
      <div className="mx-auto w-full max-w-[855px] rounded-2xl border bg-neutral-950 p-8 text-white shadow-sm">
        <div className="font-mono text-sm text-neutral-400">termui::workspace</div>
        <div className="mt-5 grid gap-3 sm:grid-cols-3">
          {[
            ["Widgets", "copyable source"],
            ["Rust", "native rendering"],
            ["Ratatui", "terminal UI"],
          ].map(([value, label]) => (
            <div key={value} className="rounded-lg border border-white/10 bg-white/5 p-4">
              <div className="text-2xl font-semibold">{value}</div>
              <div className="mt-1 text-sm text-neutral-400">{label}</div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
