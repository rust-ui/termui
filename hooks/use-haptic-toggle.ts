"use client";

import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { useCallback } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useSoundEnabled } from "@/hooks/use-sound-toggle";

const hapticsEnabledAtom = atomWithStorage("haptics-enabled", false);

export const useHapticsEnabled = () => useAtom(hapticsEnabledAtom);

export const useHapticsToggle = () => {
  const [hapticsEnabled, setHapticsEnabled] = useAtom(hapticsEnabledAtom);
  useSoundEnabled();

  const toggleHaptics = useCallback(() => {
    setHapticsEnabled(!hapticsEnabled);
  }, [hapticsEnabled, setHapticsEnabled]);

  useHotkeys(
    "h",
    () => toggleHaptics(),
    {
      enableOnContentEditable: true,
      enableOnFormTags: true,
      preventDefault: true,
    },
    [toggleHaptics]
  );

  return { hapticsEnabled, toggleHaptics };
};
