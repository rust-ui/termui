"use client";

import { useCallback } from "react";
export type FeedbackType = string;

export interface UseFeedbackOptions {
  sound?: FeedbackType;
  soundDef?: unknown;
  haptic?: boolean;
}

export const useFeedback = ({
  sound,
  soundDef,
  haptic = true,
}: UseFeedbackOptions) => {
  return useCallback(() => {}, [sound, soundDef, haptic]);
};
