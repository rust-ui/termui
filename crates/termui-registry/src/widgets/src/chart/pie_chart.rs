use core::f64::consts::TAU;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use ratatui::widgets::{Block, Paragraph, Wrap};

/// One proportional slice in a [`PieChart`].
#[derive(Clone, Copy, Debug)]
pub struct PieSlice<'a> {
    label: &'a str,
    value: f64,
    color: Color,
}

impl<'a> PieSlice<'a> {
    /// Create a labeled slice from a non-negative value.
    pub const fn new(label: &'a str, value: f64, color: Color) -> Self {
        Self {
            label,
            value,
            color,
        }
    }
}

/// Proportional pie or donut chart with a composable legend.
#[must_use]
pub struct PieChart<'a> {
    slices: &'a [PieSlice<'a>],
    title: Option<&'a str>,
    donut: bool,
    legend: bool,
}

impl<'a> PieChart<'a> {
    /// Create a pie chart from labeled slices.
    pub const fn new(slices: &'a [PieSlice<'a>]) -> Self {
        Self {
            slices,
            title: None,
            donut: false,
            legend: true,
        }
    }

    /// Render a hollow center while keeping the same proportional slices.
    pub const fn donut(mut self, donut: bool) -> Self {
        self.donut = donut;
        self
    }

    /// Show or hide the slice legend.
    pub const fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }

    /// Add a title to the chart frame.
    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let block = self.title.map(|title| Block::bordered().title(title));
        let inner = block.as_ref().map_or(area, |block| block.inner(area));
        if let Some(block) = block {
            frame.render_widget(block, area);
        }
        let (chart_area, legend_area) = if self.legend {
            let [chart, legend] =
                Layout::horizontal([Constraint::Percentage(68), Constraint::Percentage(32)])
                    .areas(inner);
            (chart, Some(legend))
        } else {
            (inner, None)
        };

        let total = self
            .slices
            .iter()
            .map(|slice| slice.value.max(0.0))
            .sum::<f64>()
            .max(f64::EPSILON);
        let slices = self.slices;
        let inner_radius = if self.donut { 0.48 } else { 0.0 };
        let canvas = Canvas::default()
            .marker(Marker::HalfBlock)
            .x_bounds([-1.0, 1.0])
            .y_bounds([-0.5, 0.5])
            .paint(move |ctx| {
                let mut start = -core::f64::consts::FRAC_PI_2;
                for slice in slices.iter().filter(|slice| slice.value > 0.0) {
                    let end = start + TAU * slice.value / total;
                    ctx.draw(&PieWedge {
                        start,
                        end,
                        inner_radius,
                        color: slice.color,
                    });
                    start = end;
                }
            });
        frame.render_widget(canvas, chart_area);

        if let Some(legend_area) = legend_area {
            let lines = self
                .slices
                .iter()
                .map(|slice| {
                    let percent = slice.value.max(0.0) * 100.0 / total;
                    Line::from(vec![
                        Span::styled("● ", Style::default().fg(slice.color)),
                        Span::raw(format!("{}  {:.0}%", slice.label, percent)),
                    ])
                })
                .collect::<Vec<_>>();
            frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), legend_area);
        }
    }
}

struct PieWedge {
    start: f64,
    end: f64,
    inner_radius: f64,
    color: Color,
}

impl Shape for PieWedge {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let angle_step = 0.035;
        let radius_step = 0.025;
        let mut angle = self.start;
        while angle <= self.end {
            let mut radius = self.inner_radius;
            while radius <= 1.0 {
                let x = radius * angle.cos();
                let y = radius * angle.sin();
                if let Some((x, y)) = painter.get_point(x, y) {
                    painter.paint(x, y, self.color);
                }
                radius += radius_step;
            }
            angle += angle_step;
        }
    }
}
