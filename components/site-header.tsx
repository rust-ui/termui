import Link from "next/link";

import { LogoMark, LogoType } from "@/components/logo";
import { MainNav } from "@/components/main-nav";
import { MobileNav } from "@/components/mobile-nav";
import { ModeSwitcher } from "@/components/mode-switcher";
import { NavItemGithub } from "@/components/nav-item-github";
import { NAV_ITEMS, TOP_LEVEL_SECTIONS } from "@/constants/nav";
import { ROUTES } from "@/constants/routes";

export const SiteHeader = () => (
  <header className="border-grid bg-background/95 supports-backdrop-filter:bg-background/60 sticky top-0 z-50 w-full border-b backdrop-blur-sm">
    <div className="container-wrapper">
      <div className="container flex h-14 items-center gap-2 md:gap-4">
        <Link href={ROUTES.HOME} className="mr-2 flex items-center gap-2">
          <LogoMark />
          <LogoType />
        </Link>
        <MainNav items={NAV_ITEMS} className="hidden md:flex" />
        <div className="ml-auto flex items-center gap-1">
          <NavItemGithub />
          <ModeSwitcher />
          <MobileNav items={TOP_LEVEL_SECTIONS} className="md:hidden" />
        </div>
      </div>
    </div>
  </header>
);
