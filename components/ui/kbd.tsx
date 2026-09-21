import type * as React from "react";

import { cn } from "@/lib/utils";

export const Kbd = ({ className, ...props }: React.ComponentProps<"kbd">) => (
  <kbd
    data-slot="kbd"
    className={cn(
      "bg-muted text-muted-foreground pointer-events-none inline-flex h-5 items-center gap-1 rounded border px-1.5 font-mono text-[10px] font-medium select-none",
      className
    )}
    {...props}
  />
);
