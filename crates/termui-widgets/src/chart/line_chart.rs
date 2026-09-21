use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Block, Chart as RatatuiChart, Dataset, GraphType};

use super::ChartSeries;

/// Multi-series line chart with optional point markers and step interpolation.
#[must_use]
pub struct LineChart<'a> {
    series: Vec<ChartSeries<'a>>,
    title: Option<&'a str>,
    markers: bool,
    step: bool,
    legend: bool,
}

impl<'a> LineChart<'a> {
    /// Create an empty chart; add data with [`LineChart::series`].
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            title: None,
            markers: false,
            step: false,
            legend: true,
        }
    }

    /// Add a named line.
    pub fn series(mut self, series: ChartSeries<'a>) -> Self {
        self.series.push(series);
        self
    }

    /// Add a title to the chart frame.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Draw a marker at every data point.
    pub const fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }

    /// Connect points with horizontal and vertical segments.
    pub const fn step(mut self, step: bool) -> Self {
        self.step = step;
        self
    }

    /// Show each series name in the chart legend.
    pub const fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let all_points = self
            .series
            .iter()
            .map(|series| {
                if self.step {
                    let mut points = Vec::with_capacity(series.points.len().saturating_mul(2));
                    for pair in series.points.windows(2) {
                        points.push(pair[0]);
                        points.push((pair[1].0, pair[0].1));
                    }
                    if let Some(last) = series.points.last() {
                        points.push(*last);
                    }
                    points
                } else {
                    series.points.to_vec()
                }
            })
            .collect::<Vec<_>>();

        let mut datasets = self
            .series
            .iter()
            .zip(&all_points)
            .map(|(series, points)| {
                Dataset::default()
                    .name(series.name)
                    .data(points)
                    .graph_type(GraphType::Line)
                    .marker(if self.markers {
                        Marker::Dot
                    } else {
                        Marker::Braille
                    })
                    .style(Style::default().fg(series.color))
            })
            .collect::<Vec<_>>();
        if self.markers {
            datasets.extend(self.series.iter().map(|series| {
                Dataset::default()
                    .data(series.points)
                    .graph_type(GraphType::Scatter)
                    .marker(Marker::Block)
                    .style(Style::default().fg(series.color))
            }));
        }

        let (x_min, x_max, y_min, y_max) = bounds(&self.series);
        let mut chart = RatatuiChart::new(datasets)
            .x_axis(Axis::default().bounds([x_min, x_max]))
            .y_axis(Axis::default().bounds([y_min, y_max]));
        if let Some(title) = self.title {
            chart = chart.block(Block::bordered().title(title));
        }
        if !self.legend {
            chart = chart.legend_position(None);
        }
        frame.render_widget(chart, area);
    }
}

impl Default for LineChart<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) fn bounds(series: &[ChartSeries<'_>]) -> (f64, f64, f64, f64) {
    let mut points = series.iter().flat_map(|series| series.points.iter());
    let Some(first) = points.next() else {
        return (0.0, 1.0, 0.0, 1.0);
    };
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (first.0, first.0, first.1, first.1);
    for (x, y) in points {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        y_min = y_min.min(*y);
        y_max = y_max.max(*y);
    }
    if x_min == x_max {
        x_max += 1.0;
    }
    if y_min == y_max {
        y_max += 1.0;
    }
    (x_min, x_max, y_min.min(0.0), y_max)
}
