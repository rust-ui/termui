use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

#[path = "chart/area_chart.rs"]
pub mod area_chart;
#[path = "chart/bar_chart.rs"]
pub mod bar_chart;
#[path = "chart/chart_tooltip.rs"]
pub mod chart_tooltip;
#[path = "chart/line_chart.rs"]
pub mod line_chart;
#[path = "chart/pie_chart.rs"]
pub mod pie_chart;
#[path = "chart/radar_chart.rs"]
pub mod radar_chart;
#[path = "chart/radial_chart.rs"]
pub mod radial_chart;
mod series;

pub use bar_chart::{BarChart, BarOrientation};
pub use series::ChartSeries;

/// A compact horizontal bar chart for labeled values.
#[must_use]
pub struct Chart<'a> {
    values: &'a [(&'a str, u64)],
}

impl<'a> Chart<'a> {
    pub fn new(values: &'a [(&'a str, u64)]) -> Self {
        Self { values }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let max = self
            .values
            .iter()
            .map(|(_, value)| *value)
            .max()
            .unwrap_or(0)
            .max(1);
        let label_width = self
            .values
            .iter()
            .map(|(label, _)| label.len())
            .max()
            .unwrap_or(1)
            .min(10);
        let value_width = max.to_string().len();
        let bar_width = (area.width as usize).saturating_sub(label_width + value_width + 3);
        let lines = self
            .values
            .iter()
            .map(|(label, value)| {
                let width = ((*value as usize * bar_width) / max as usize).min(bar_width);
                let text = format!("{label:>label_width$} {} {value}", "█".repeat(width));
                Line::from(Span::styled(text, Style::default().fg(Color::Cyan)))
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
