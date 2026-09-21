"use client";

import { MonitorIcon, MoonIcon, SunIcon } from "lucide-react";
import { useTheme } from "next-themes";

import { useMounted } from "@/hooks/use-mounted";
import { cn } from "@/lib/utils";

const THEME_OPTIONS = [
  { icon: MonitorIcon, value: "system" },
  { icon: SunIcon, value: "light" },
  { icon: MoonIcon, value: "dark" },
] as const;

export const ModeSwitcher = () => {
  const { theme, setTheme } = useTheme();
  const isMounted = useMounted();

  if (!isMounted) {
    return <div className="flex h-8 w-24" />;
  }

  return (
    <div
      className="bg-background inset-ring-border inset-ring-1 inline-flex items-center rounded-full"
      role="radiogroup"
      aria-label="Theme"
    >
      {THEME_OPTIONS.map((option) => {
        const Icon = option.icon;
        const isActive = theme === option.value;

        return (
          <button
            key={option.value}
            type="button"
            data-active={isActive}
            className={cn(
              "text-muted-foreground hover:text-foreground data-[active=true]:text-foreground data-[active=true]:inset-ring-border relative flex size-8 items-center justify-center rounded-full transition-[color,box-shadow] data-[active=true]:inset-ring-1 [&_svg]:size-4"
            )}
            role="radio"
            aria-checked={isActive}
            aria-label={`Switch to ${option.value} theme`}
            onClick={() => setTheme(option.value)}
          >
            <Icon />
          </button>
        );
      })}
    </div>
  );
};
