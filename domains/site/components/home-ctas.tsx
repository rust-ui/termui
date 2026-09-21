"use client";

import Link from "next/link";

import { Button } from "@/components/ui/button";
import { ROUTES } from "@/shared/config/routes";
import { RATATUI_WIDGETS_TITLE } from "@/domains/ratatui/config";
import { useIconAnimation } from "@/shared/hooks/use-icon-animation";
import { cn } from "@/shared/lib/utils";

import { ArrowRightIcon } from "@/components/shared/animated-icons/arrow-right";
import type { ArrowRightIconHandle } from "@/components/shared/animated-icons/arrow-right";
import { ComponentIcon } from "@/components/shared/animated-icons/component";
import type { ComponentIconHandle } from "@/components/shared/animated-icons/component";

const GetStartedButton = () => {
  const { iconRef, onMouseEnter, onMouseLeave } =
    useIconAnimation<ArrowRightIconHandle>();

  return (
    <Button
      asChild
      sound="click"
      className="px-4"
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <Link href={ROUTES.DOCS_INSTALLATION} transitionTypes={["nav-forward"]}>
        Get Started
        <ArrowRightIcon className="hidden sm:inline" ref={iconRef} />
      </Link>
    </Button>
  );
};

const WidgetsButton = () => {
  const { iconRef, onMouseEnter, onMouseLeave } = useIconAnimation<ComponentIconHandle>();

  return (
    <Button
      asChild
      variant="outline"
      sound="click"
      className="px-4"
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <Link href={ROUTES.DOCS_WIDGETS} transitionTypes={["nav-forward"]}>
        <ComponentIcon className="hidden sm:inline" ref={iconRef} size={22} />
        {RATATUI_WIDGETS_TITLE}
      </Link>
    </Button>
  );
};

export const HomeCtas = ({ className }: { className?: string }) => (
  <div className={cn("flex flex-wrap items-center justify-center gap-4", className)}>
    <GetStartedButton />
    <WidgetsButton />
  </div>
);
