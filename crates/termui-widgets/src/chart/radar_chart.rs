use core::f64::consts::TAU;

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::canvas::{Canvas, Circle, Line};
use ratatui::widgets::{Block, Paragraph};

/// One labeled axis value in a radar chart.
#[derive(Clone, Copy, Debug)]
pub struct RadarValue<'a> {
    pub label: &'a str,
    pub value: f64,
}

/// One polygon series in a [`RadarChart`].
#[derive(Clone, Copy, Debug)]
pub struct RadarSeries<'a> {
    pub name: &'a str,
    pub values: &'a [RadarValue<'a>],
    pub color: Color,
}

impl<'a> RadarSeries<'a> {
    /// Create a series with one value for each radar axis.
    pub const fn new(name: &'a str, values: &'a [RadarValue<'a>]) -> Self {
        Self {
            name,
            values,
            color: Color::Cyan,
        }
    }

    /// Set the series color.
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// Polar comparison chart with independent series and grid rings.
#[must_use]
pub struct RadarChart<'a> {
    series: Vec<RadarSeries<'a>>,
    max: f64,
    title: Option<&'a str>,
}

impl<'a> RadarChart<'a> {
    /// Create an empty radar chart; add polygons with [`RadarChart::series`].
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            max: 10.0,
            title: None,
        }
    }

    /// Add a labeled data polygon.
    pub fn series(mut self, series: RadarSeries<'a>) -> Self {
        self.series.push(series);
        self
    }

    /// Set the maximum value represented by the outer grid ring.
    pub const fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// Add a title to the chart frame.
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let series = self.series;
        let max = self.max.max(f64::EPSILON);
        let block = self.title.map(|title| Block::bordered().title(title));
        let inner = block.as_ref().map_or(area, |block| block.inner(area));
        if let Some(block) = block {
            frame.render_widget(block, area);
        }
        let (chart_area, legend_area) = if !series.is_empty() && inner.height >= 5 {
            let [chart, legend] =
                Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(inner);
            (chart, Some(legend))
        } else {
            (inner, None)
        };
        let legend = series
            .iter()
            .flat_map(|item| {
                [
                    Span::styled("● ", ratatui::style::Style::default().fg(item.color)),
                    Span::raw(item.name),
                    Span::raw("  "),
                ]
            })
            .collect::<Vec<_>>();
        let canvas = Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-1.25, 1.25])
            .y_bounds([-0.625, 0.625])
            .paint(move |ctx| {
                for ring in [0.33, 0.66, 1.0] {
                    ctx.draw(&Circle::new(0.0, 0.0, ring, Color::DarkGray));
                }
                let axes = series
                    .iter()
                    .map(|item| item.values.len())
                    .max()
                    .unwrap_or(0);
                if axes < 3 {
                    return;
                }
                for index in 0..axes {
                    let angle = TAU * index as f64 / axes as f64 - core::f64::consts::FRAC_PI_2;
                    let (x, y) = (angle.cos(), angle.sin());
                    ctx.draw(&Line::new(0.0, 0.0, x, y, Color::DarkGray));
                    if let Some(label) = series.first().and_then(|set| set.values.get(index)) {
                        ctx.print(x * 1.12, y * 1.12, TextLine::from(label.label.to_owned()));
                    }
                }
                for set in &series {
                    let count = set.values.len();
                    if count < 3 {
                        continue;
                    }
                    let points = set
                        .values
                        .iter()
                        .enumerate()
                        .map(|(index, value)| {
                            let angle =
                                TAU * index as f64 / count as f64 - core::f64::consts::FRAC_PI_2;
                            let radius = (value.value / max).clamp(0.0, 1.0);
                            (angle.cos() * radius, angle.sin() * radius)
                        })
                        .collect::<Vec<_>>();
                    for index in 0..points.len() {
                        let (x1, y1) = points[index];
                        let (x2, y2) = points[(index + 1) % points.len()];
                        ctx.draw(&Line::new(x1, y1, x2, y2, set.color));
                    }
                }
            });
        frame.render_widget(canvas, chart_area);

        if let Some(legend_area) = legend_area {
            frame.render_widget(
                Paragraph::new(TextLine::from(legend)).alignment(Alignment::Center),
                legend_area,
            );
        }
    }
}

impl Default for RadarChart<'_> {
    fn default() -> Self {
        Self::new()
    }
}
