use core::f64::consts::{FRAC_PI_2, TAU};

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::text::Line as TextLine;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Circle, Line};

/// Single progress value rendered as a radial bar.
#[must_use]
pub struct RadialChart<'a> {
    label: &'a str,
    value: f64,
    max: f64,
    color: Color,
    title: Option<&'a str>,
}

impl<'a> RadialChart<'a> {
    /// Create a radial chart with a value and its scale maximum.
    pub const fn new(label: &'a str, value: f64, max: f64) -> Self {
        Self {
            label,
            value,
            max,
            color: Color::Cyan,
            title: None,
        }
    }

    /// Set the radial bar color.
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Add a title to the chart frame.
    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let ratio = (self.value / self.max.max(f64::EPSILON)).clamp(0.0, 1.0);
        let label = self.label;
        let color = self.color;
        let percent = format!("{:.0}%", ratio * 100.0);
        let canvas = Canvas::default()
            .marker(Marker::HalfBlock)
            .x_bounds([-1.15, 1.15])
            .y_bounds([-0.575, 0.575])
            .paint(move |ctx| {
                ctx.draw(&Circle::new(0.0, 0.0, 0.9, Color::DarkGray));
                let steps = (ratio * 120.0).ceil() as usize;
                for step in 0..steps {
                    let angle = -FRAC_PI_2 + TAU * step as f64 / 120.0;
                    let (sin, cos) = angle.sin_cos();
                    ctx.draw(&Line::new(
                        cos * 0.75,
                        sin * 0.75,
                        cos * 1.05,
                        sin * 1.05,
                        color,
                    ));
                }
                ctx.print(-0.16, 0.02, TextLine::from(percent.clone()));
                ctx.print(-0.32, -0.2, TextLine::from(label.to_owned()));
            });
        if let Some(title) = self.title {
            frame.render_widget(canvas.block(Block::bordered().title(title)), area);
        } else {
            frame.render_widget(canvas, area);
        }
    }
}
