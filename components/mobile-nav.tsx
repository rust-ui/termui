"use client";

import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle } from "@/components/ui/sheet";
import { cn } from "@/lib/utils";

const MobileLink = ({
  href,
  onOpenChange,
  className,
  children,
}: {
  href: string;
  onOpenChange: (open: boolean) => void;
  className?: string;
  children: React.ReactNode;
}) => {
  const router = useRouter();

  return (
    <Link
      href={href}
      onClick={() => {
        router.push(href);
        onOpenChange(false);
      }}
      className={className}
    >
      {children}
    </Link>
  );
};

export const MobileNav = ({
  items,
  className,
}: {
  items: { href: string; name: string }[];
  className?: string;
}) => {
  const [open, setOpen] = useState(false);
  const pathname = usePathname();

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <Button
        variant="ghost"
        size="icon"
        className={cn("relative", className)}
        aria-label="Toggle Menu"
        onClick={() => setOpen((prev) => !prev)}
      >
        <span className="relative flex size-4 items-center justify-center">
          <span
            className={cn(
              "bg-foreground absolute h-px w-4 transition-transform",
              open ? "rotate-45" : "-translate-y-1"
            )}
          />
          <span
            className={cn(
              "bg-foreground absolute h-px w-4 transition-transform",
              open ? "-rotate-45" : "translate-y-1"
            )}
          />
        </span>
      </Button>
      <SheetContent side="right" className="w-72 p-0">
        <SheetHeader className="sr-only">
          <SheetTitle>Menu</SheetTitle>
          <SheetDescription>Site navigation.</SheetDescription>
        </SheetHeader>
        <div className="flex flex-col gap-1 p-4">
          {items.map((item) => (
            <MobileLink
              key={item.href}
              href={item.href}
              onOpenChange={setOpen}
              className={cn(
                "rounded-md px-3 py-2 text-sm",
                pathname === item.href ? "bg-accent text-accent-foreground" : "text-muted-foreground"
              )}
            >
              {item.name}
            </MobileLink>
          ))}
        </div>
      </SheetContent>
    </Sheet>
  );
};
