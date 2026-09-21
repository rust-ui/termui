export const RATATUI_DEMO_BASE = "rust";
export const RATATUI_WIDGETS_TITLE = "Widgets";

export const RATATUI_COMPONENTS = [
  {
    description: "Shadcn-inspired terminal button with six variants and three sizes.",
    name: "button",
    source: "crates/termui-widgets/src/button.rs",
    title: "Button",
  },
  {
    description: "Composable bordered card with header, action, content, and footer regions.",
    name: "card",
    source: "crates/termui-widgets/src/card.rs",
    title: "Card",
  },
  {
    description: "Rounded panel shell for Ratatui widgets, with optional top and bottom titles.",
    name: "panel",
    source: "crates/termui-widgets/src/panel.rs",
    title: "Panel",
  },
  {
    description: "Selectable terminal rows with a full-width active-row highlight.",
    name: "select-list",
    source: "crates/termui-widgets/src/select_list.rs",
    title: "Select List",
  },
] as const;

export type RatatuiComponent = (typeof RATATUI_COMPONENTS)[number];
export type RatatuiComponentName = RatatuiComponent["name"];
export type RatatuiComponentSource = RatatuiComponent["source"];
export type ComponentPreviewBase = typeof RATATUI_DEMO_BASE;
