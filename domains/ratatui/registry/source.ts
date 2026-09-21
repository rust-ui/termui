import { RATATUI_DEMO_BASE } from "@/domains/ratatui/config";
import type { RatatuiComponentName } from "@/domains/ratatui/config";

export const getDemoSource = (name: RatatuiComponentName): string =>
  `use termui_renderer::render_demo;\n\nfn main() {\n    let frame = render_demo("${RATATUI_DEMO_BASE}/${name}");\n    println!("{}", frame.join("\\n"));\n}\n`;
