"use client";

import { Code, Eye } from "lucide-react";

import { CopyButton } from "@/components/copy-button";
import { RatatuiDemoPreview } from "@/components/ratatui-demo-preview";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface RustDemoProps {
  name: string;
  code: string;
  src?: string;
  title?: string;
  rows?: number;
  height?: number;
  fontSize?: number;
}

/** Preview/code switcher for checked-in Rust-rendered widget examples. */
export function RustDemo({
  name,
  code,
  src,
  title = name,
  rows = 18,
  height = 370,
  fontSize = 12,
}: RustDemoProps) {
  return (
    <Tabs defaultValue="preview" className="my-6 gap-2">
      <TabsList className="self-start">
        <TabsTrigger value="preview">
          <Eye className="size-3.5" />
          Preview
        </TabsTrigger>
        <TabsTrigger value="code">
          <Code className="size-3.5" />
          Code
        </TabsTrigger>
      </TabsList>
      <TabsContent value="preview">
        <div className="flex min-h-[370px] items-center justify-center overflow-hidden rounded-xl border bg-background p-4">
          {src ? (
            <iframe
              src={src}
              title={title}
              loading="lazy"
              className="w-full border-0"
              style={{ height }}
            />
          ) : (
            <RatatuiDemoPreview name={name} rows={rows} fontSize={fontSize} />
          )}
        </div>
      </TabsContent>
      <TabsContent value="code">
        <div className="relative min-h-[370px] overflow-hidden rounded-xl border bg-muted">
          <CopyButton value={code} event="copy_primitive_code" />
          <pre className="m-0 min-h-[370px] overflow-x-auto whitespace-pre p-4 font-mono text-xs leading-relaxed">
            <code>{code}</code>
          </pre>
        </div>
      </TabsContent>
    </Tabs>
  );
}
