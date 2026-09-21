import Link from "next/link";

import { BrandContextMenu } from "@/components/brand-context-menu";
import { CommandMenu } from "@/components/command-menu";
import { LabsNav } from "@/components/labs-nav";
import { LogoMark } from "@/components/logo";
import { MainNav } from "@/components/main-nav";
import { MobileNav } from "@/components/mobile-nav";
import { NavItemGithub } from "@/components/nav-item-github";
import { ThemeToggle } from "@/components/theme-toggle";
import { Button } from "@/components/ui/button";
import { ROUTES } from "@/constants/routes";
import { RATATUI_WIDGETS_TITLE } from "@/constants/ratatui";
import { source } from "@/lib/source";

const navItems = [
  { href: ROUTES.DOCS, label: "Docs" },
  { href: ROUTES.DOCS_WIDGETS, label: RATATUI_WIDGETS_TITLE },
  { href: ROUTES.CHARTS, label: "Charts" },
  { href: ROUTES.DOCS_INSTALLATION, label: "Installation" },
  { href: ROUTES.DOCS_MCP, label: "MCP" },
];

export const SiteHeader = () => (
  <header
    className="bg-background sticky top-0 z-50 w-full"
    style={{ viewTransitionName: "site-header" }}
  >
    <div className="container-wrapper 3xl:fixed:px-0 relative px-6">
      <div className="3xl:fixed:container relative flex h-(--header-height) items-center">
        <MobileNav
          items={navItems}
          tree={source.pageTree}
          className="flex lg:hidden mr-2"
        />
        <div className="flex items-center">
          <BrandContextMenu>
            <Button
              asChild
              variant="ghost"
              size="icon-sm"
              className="hover:bg-transparent focus-visible:bg-transparent dark:hover:bg-transparent lg:size-9"
              sound="click"
            >
              <Link href="/">
                <LogoMark className="size-5" />
              </Link>
            </Button>
          </BrandContextMenu>
          <LabsNav />
          <span aria-hidden="true" className="text-muted-foreground/50">
            /
          </span>
        </div>
        <MainNav items={navItems} className="hidden lg:flex" />
        <div className="ml-auto flex items-center gap-1 md:gap-2 md:flex-1 md:justify-end">
          <div className="hidden w-full flex-1 md:flex md:w-auto md:flex-none">
            <CommandMenu navItems={navItems} tree={source.pageTree} />
          </div>
          <NavItemGithub />
          <ThemeToggle />
        </div>
      </div>
    </div>
  </header>
);
