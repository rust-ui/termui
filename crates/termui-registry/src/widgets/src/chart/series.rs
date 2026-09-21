use ratatui::style::Color;

/// Named numeric series shared by line, area, and radar charts.
#[derive(Clone, Copy, Debug)]
pub struct ChartSeries<'a> {
    pub(super) name: &'a str,
    pub(super) points: &'a [(f64, f64)],
    pub(super) color: Color,
}

impl<'a> ChartSeries<'a> {
    /// Create a named series from ordered `(x, y)` data points.
    pub const fn new(name: &'a str, points: &'a [(f64, f64)]) -> Self {
        Self {
            name,
            points,
            color: Color::Cyan,
        }
    }

    /// Set the series color.
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
