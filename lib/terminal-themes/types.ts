export interface Theme {
  border: {
    color: string;
    focusColor: string;
    style: string;
  };
  colors: {
    accent: string;
    accentForeground: string;
    background: string;
    border: string;
    error: string;
    errorForeground: string;
    focusRing: string;
    foreground: string;
    info: string;
    infoForeground: string;
    muted: string;
    mutedForeground: string;
    primary: string;
    primaryForeground: string;
    secondary: string;
    secondaryForeground: string;
    selection: string;
    selectionForeground: string;
    success: string;
    successForeground: string;
    warning: string;
    warningForeground: string;
  };
  name: string;
  spacing: {
    0: number;
    1: number;
    2: number;
    3: number;
    4: number;
    6: number;
    8: number;
  };
  typography: {
    base: string;
    bold: boolean;
    lg: string;
    sm: string;
    xl: string;
  };
}

export type TerminalTheme = Theme;
