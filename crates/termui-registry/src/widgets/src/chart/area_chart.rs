use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Chart as RatatuiChart, Dataset, GraphType};

use super::ChartSeries;
use super::line_chart::bounds;

/// Filled multi-series area chart.
#[must_use]
pub struct AreaChart<'a> {
    series: Vec<ChartSeries<'a>>,
    title: Option<&'a str>,
    markers: bool,
    fill_to: f64,
}

impl<'a> AreaChart<'a> {
    /// Create an empty chart; add data with [`AreaChart::series`].
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            title: None,
            markers: false,
            fill_to: 0.0,
        }
    }

    /// Add a named area.
    pub fn series(mut self, series: ChartSeries<'a>) -> Self {
        self.series.push(series);
        self
    }

    /// Add a title to the chart frame.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the y-value used as the fill baseline.
    pub const fn fill_to(mut self, value: f64) -> Self {
        self.fill_to = value;
        self
    }

    /// Draw markers on top of each series.
    pub const fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let (x_min, x_max, y_min, y_max) = bounds(&self.series);
        let datasets = self
            .series
            .iter()
            .map(|series| {
                Dataset::default()
                    .name(series.name)
                    .data(series.points)
                    .graph_type(GraphType::Area)
                    .fill_to_y(self.fill_to)
                    .marker(if self.markers {
                        Marker::Dot
                    } else {
                        Marker::Braille
                    })
                    .style(Style::default().fg(series.color))
            })
            .collect();
        let mut chart = RatatuiChart::new(datasets)
            .x_axis(Axis::default().bounds([x_min, x_max]))
            .y_axis(Axis::default().bounds([y_min, y_max]))
            .hidden_legend_constraints((Constraint::Percentage(50), Constraint::Percentage(50)));
        if let Some(title) = self.title {
            chart = chart.block(Block::bordered().title(title));
        }
        frame.render_widget(chart, area);
    }
}

impl Default for AreaChart<'_> {
    fn default() -> Self {
        Self::new()
    }
}
