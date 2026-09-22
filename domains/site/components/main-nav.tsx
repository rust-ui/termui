"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

import { cn } from "@/shared/lib/utils";

export const MainNav = ({
  items,
  className,
  ...props
}: React.ComponentProps<"nav"> & {
  items: { href: string; label: string }[];
}) => {
  const pathname = usePathname();

  return (
    <nav className={cn("items-center gap-1", className)} {...props}>
      {items.map((item) => (
        <Link
          key={item.href}
          href={item.href}
          className={cn("main-nav-link", pathname === item.href && "text-primary")}
        >
          <span className="main-nav-link__inner">
            <span className="main-nav-link__text">{item.label}</span>
          </span>
        </Link>
      ))}
    </nav>
  );
};
