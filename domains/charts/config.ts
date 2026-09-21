export const CHART_FAMILIES = [
  {
    type: "area",
    label: "Area Charts",
    description: "Filled series for showing change and volume over time.",
  },
  {
    type: "bar",
    label: "Bar Charts",
    description: "Compare values across categories with vertical or horizontal bars.",
  },
  {
    type: "line",
    label: "Line Charts",
    description: "Follow one or more numeric series across an ordered axis.",
  },
  {
    type: "pie",
    label: "Pie Charts",
    description: "Show how a whole divides into proportional slices or a donut.",
  },
  {
    type: "radar",
    label: "Radar Charts",
    description: "Compare several labeled measures on the same radial scale.",
  },
  {
    type: "radial",
    label: "Radial Charts",
    description: "Show progress or a single value around a circular track.",
  },
  {
    type: "tooltip",
    label: "Tooltips",
    description: "Compose a compact value panel for a selected chart point.",
  },
] as const;

export type ChartFamily = (typeof CHART_FAMILIES)[number]["type"];

export interface ChartExample {
  name: string;
  title: string;
  description: string;
  preview: string;
  code: string;
}

export const CHART_EXAMPLES: Record<ChartFamily, ChartExample[]> = {
  area: [
    {
      name: "Area · Multiple series",
      title: "Multiple Series",
      description: "Layer multiple filled datasets over the same axis.",
      preview: "chart-area",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{area_chart::AreaChart, ChartSeries};

let desktop = [(0.0, 3.0), (1.0, 6.0), (2.0, 4.0), (3.0, 8.0)];
let mobile = [(0.0, 2.0), (1.0, 3.0), (2.0, 5.0), (3.0, 4.0)];

AreaChart::new()
    .series(ChartSeries::new("Desktop", &desktop).color(Color::Cyan))
    .series(ChartSeries::new("Mobile", &mobile).color(Color::Yellow))
    .title("Page views")
    .render(frame, area);`,
    },
    {
      name: "Area · Requests by service",
      title: "Requests by Service",
      description: "Fill each series to a shared baseline.",
      preview: "chart-area-multiple",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{area_chart::AreaChart, ChartSeries};

let api = [(0.0, 2.0), (1.0, 4.0), (2.0, 3.0), (3.0, 6.0)];
let worker = [(0.0, 1.0), (1.0, 2.0), (2.0, 4.0), (3.0, 3.0)];

AreaChart::new()
    .series(ChartSeries::new("API", &api).color(Color::Cyan))
    .series(ChartSeries::new("Worker", &worker).color(Color::Magenta))
    .fill_to(0.0)
    .title("Requests by service")
    .render(frame, area);`,
    },
  ],
  bar: [
    {
      name: "Bar · Vertical",
      title: "Vertical Bars",
      description: "Compare a small set of category values.",
      preview: "chart-bar",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{BarChart, BarOrientation};

let values = [("Mon", 5), ("Tue", 8), ("Wed", 4), ("Thu", 10)];

BarChart::new(&values)
    .orientation(BarOrientation::Vertical)
    .color(Color::Cyan)
    .title("Weekly activity")
    .render(frame, area);`,
    },
    {
      name: "Bar · Horizontal",
      title: "Horizontal Bars",
      description: "Use horizontal bars when category labels need more room.",
      preview: "chart-bar-horizontal",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{BarChart, BarOrientation};

let values = [("Rust", 48), ("TypeScript", 34), ("Python", 22)];

BarChart::new(&values)
    .orientation(BarOrientation::Horizontal)
    .color(Color::Green)
    .title("Downloads by language")
    .render(frame, area);`,
    },
  ],
  line: [
    {
      name: "Line · Default",
      title: "Default Line",
      description: "Render an ordered series with Ratatui's Cartesian chart.",
      preview: "chart-line",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{line_chart::LineChart, ChartSeries};

let visitors = [(0.0, 2.0), (1.0, 4.0), (2.0, 3.0), (3.0, 7.0)];

LineChart::new()
    .series(ChartSeries::new("Visitors", &visitors).color(Color::Cyan))
    .title("Visitors")
    .render(frame, area);`,
    },
    {
      name: "Line · Multiple series",
      title: "Multiple Series",
      description: "Compare several named datasets with a shared scale.",
      preview: "chart-line-multiple",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{line_chart::LineChart, ChartSeries};

let desktop = [(0.0, 4.0), (1.0, 7.0), (2.0, 5.0), (3.0, 9.0)];
let mobile = [(0.0, 2.0), (1.0, 3.0), (2.0, 6.0), (3.0, 5.0)];

LineChart::new()
    .series(ChartSeries::new("Desktop", &desktop).color(Color::Cyan))
    .series(ChartSeries::new("Mobile", &mobile).color(Color::Yellow))
    .title("Sessions")
    .render(frame, area);`,
    },
    {
      name: "Line · Step with markers",
      title: "Step with Markers",
      description: "Show discrete changes and mark every observation.",
      preview: "chart-line-step",
      code: `use ratatui::style::Color;
use termui_widgets::chart::{line_chart::LineChart, ChartSeries};

let deployments = [(0.0, 2.0), (1.0, 4.0), (2.0, 3.0), (3.0, 7.0)];

LineChart::new()
    .series(ChartSeries::new("Deployments", &deployments).color(Color::Yellow))
    .step(true)
    .markers(true)
    .title("Deployments")
    .render(frame, area);`,
    },
  ],
  pie: [
    {
      name: "Pie · Default",
      title: "Pie",
      description: "Map each non-negative value to a colored slice.",
      preview: "chart-pie",
      code: `use ratatui::style::Color;
use termui_widgets::chart::pie_chart::{PieChart, PieSlice};

let slices = [
    PieSlice::new("Desktop", 48.0, Color::Cyan),
    PieSlice::new("Mobile", 32.0, Color::Yellow),
    PieSlice::new("Tablet", 20.0, Color::Magenta),
];

PieChart::new(&slices)
    .title("Traffic sources")
    .render(frame, area);`,
    },
    {
      name: "Pie · Donut",
      title: "Donut",
      description: "Keep the legend while opening a center for a total label.",
      preview: "chart-pie-donut",
      code: `use ratatui::style::Color;
use termui_widgets::chart::pie_chart::{PieChart, PieSlice};

let slices = [
    PieSlice::new("Core", 55.0, Color::Cyan),
    PieSlice::new("Docs", 30.0, Color::Yellow),
    PieSlice::new("Other", 15.0, Color::Magenta),
];

PieChart::new(&slices)
    .donut(true)
    .title("Package downloads")
    .render(frame, area);`,
    },
  ],
  radar: [
    {
      name: "Radar · Single series",
      title: "Single Series",
      description: "Compare labeled measures against one shared maximum.",
      preview: "chart-radar",
      code: `use ratatui::style::Color;
use termui_widgets::chart::radar_chart::{RadarChart, RadarSeries, RadarValue};

let profile = [
    RadarValue { label: "Speed", value: 8.0 },
    RadarValue { label: "Safety", value: 7.0 },
    RadarValue { label: "DX", value: 9.0 },
    RadarValue { label: "Docs", value: 6.0 },
];

RadarChart::new()
    .series(RadarSeries::new("Term/UI", &profile).color(Color::Cyan))
    .max(10.0)
    .title("Crate profile")
    .render(frame, area);`,
    },
    {
      name: "Radar · Compare",
      title: "Compare",
      description: "Overlay multiple profiles to compare the same axes.",
      preview: "chart-radar-compare",
      code: `use ratatui::style::Color;
use termui_widgets::chart::radar_chart::{RadarChart, RadarSeries, RadarValue};

let termui = [
    RadarValue { label: "Speed", value: 8.0 },
    RadarValue { label: "Safety", value: 9.0 },
    RadarValue { label: "DX", value: 7.0 },
    RadarValue { label: "Docs", value: 6.0 },
];
let ratatui = [
    RadarValue { label: "Speed", value: 7.0 },
    RadarValue { label: "Safety", value: 8.0 },
    RadarValue { label: "DX", value: 9.0 },
    RadarValue { label: "Docs", value: 8.0 },
];

let chart = RadarChart::new()
    .series(RadarSeries::new("Term/UI", &termui).color(Color::Cyan))
    .series(RadarSeries::new("Ratatui", &ratatui).color(Color::Yellow))
    .max(10.0)
    .title("Framework comparison");

chart.render(frame, area);`,
    },
  ],
  radial: [
    {
      name: "Radial · Progress",
      title: "Progress",
      description: "Show one bounded value around a radial track.",
      preview: "chart-radial",
      code: `use ratatui::style::Color;
use termui_widgets::chart::radial_chart::RadialChart;

RadialChart::new("CPU", 72.0, 100.0)
    .color(Color::Green)
    .title("System load")
    .render(frame, area);`,
    },
  ],
  tooltip: [
    {
      name: "Tooltip · Series values",
      title: "Series Values",
      description: "Compose a small selection panel from named colored values.",
      preview: "chart-tooltip",
      code: `use ratatui::style::Color;
use termui_widgets::chart::chart_tooltip::{ChartTooltip, TooltipEntry};

let entries = [
    TooltipEntry::new("Desktop", "1,284", Color::Cyan),
    TooltipEntry::new("Mobile", "946", Color::Yellow),
];

ChartTooltip::new("Apr 18", &entries).render(frame, area);`,
    },
  ],
};

export const getChartFamily = (type: string) =>
  CHART_FAMILIES.find((family) => family.type === type);
