"use client";

import { Code2Icon, FileCode2Icon } from "lucide-react";
import { useState } from "react";

import { CopyButton } from "@/components/copy-button";
import { RatatuiDemoPreview } from "@/components/ratatui-demo-preview";
import { Button } from "@/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet";
import type { ChartExample } from "@/constants/charts";

export function ChartGallery({
  description,
  examples,
  title,
}: {
  description: string;
  examples: ChartExample[];
  title: string;
}) {
  const [selected, setSelected] = useState<ChartExample | null>(null);

  return (
    <section className="pt-10" id="charts" aria-labelledby="chart-family-title">
      <div className="mb-6 flex flex-wrap items-end justify-between gap-3">
        <div>
          <h2 className="text-xl font-semibold tracking-tight" id="chart-family-title">
            {title}
          </h2>
          <p className="text-muted-foreground mt-1 text-sm">{description}</p>
        </div>
        <span className="text-muted-foreground text-xs">
          {examples.length} {examples.length === 1 ? "example" : "examples"}
        </span>
      </div>

      <div className="grid gap-5 md:grid-cols-2">
        {examples.map((example) => (
          <article className="min-w-0" key={example.preview}>
            <div className="flex h-11 items-center justify-between gap-2 px-1">
              <div className="min-w-0">
                <h3 className="truncate text-sm font-medium">{example.title}</h3>
                <p className="text-muted-foreground truncate text-xs">
                  {example.description}
                </p>
              </div>
              <span className="text-muted-foreground inline-flex shrink-0 items-center gap-1.5 text-xs">
                <span
                  aria-hidden="true"
                  className="size-1.5 rounded-full bg-muted-foreground"
                />
                Static
              </span>
              <Button
                className="shrink-0"
                onClick={() => setSelected(example)}
                size="sm"
                variant="outline"
              >
                <Code2Icon />
                View Code
              </Button>
            </div>
            <div className="mt-2 flex h-[290px] items-center justify-center overflow-hidden rounded-xl border bg-black p-4">
              <div className="w-full overflow-hidden rounded-md">
                <RatatuiDemoPreview fontSize={10} name={example.preview} rows={14} />
              </div>
            </div>
          </article>
        ))}
      </div>

      <Sheet
        open={selected !== null}
        onOpenChange={(open) => {
          if (!open) setSelected(null);
        }}
      >
        {selected && (
          <SheetContent className="gap-0 p-0 sm:max-w-full md:w-[760px] md:max-w-[760px]">
            <SheetHeader className="sr-only">
              <SheetTitle>{selected.title} code</SheetTitle>
              <SheetDescription>
                Preview and copy the Rust source for this chart.
              </SheetDescription>
            </SheetHeader>
            <div className="hidden shrink-0 border-b bg-black p-6 sm:block">
              <RatatuiDemoPreview fontSize={10} name={selected.preview} rows={12} />
            </div>
            <div className="flex min-h-0 flex-1 flex-col p-4 sm:p-6">
              <div className="flex h-12 shrink-0 items-center gap-2 border-b px-1 text-sm font-medium">
                <FileCode2Icon className="text-muted-foreground size-4" />
                {selected.title}.rs
                <CopyButton
                  className="ml-auto"
                  event="copy_primitive_code"
                  value={selected.code}
                  variant="outline"
                >
                  Copy
                </CopyButton>
              </div>
              <pre className="bg-muted/50 mt-4 min-h-0 flex-1 overflow-auto rounded-lg border p-4 font-mono text-xs leading-relaxed">
                <code>{selected.code}</code>
              </pre>
            </div>
          </SheetContent>
        )}
      </Sheet>
    </section>
  );
}
