"use client";

import { useCallback } from "react";
export type FeedbackType = string;

export interface UseFeedbackOptions {
  sound?: FeedbackType;
  soundDef?: unknown;
  haptic?: boolean;
}

export const useFeedback = (_options: UseFeedbackOptions) => useCallback(() => {}, []);
