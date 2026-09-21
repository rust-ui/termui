use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Bar as RatatuiBar, BarChart as RatatuiBarChart, Block};

/// Orientation for a [`BarChart`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarOrientation {
    /// Columns rise from the bottom axis.
    #[default]
    Vertical,
    /// Bars extend from the left axis.
    Horizontal,
}

/// Labeled categorical values rendered as grouped terminal bars.
#[must_use]
pub struct BarChart<'a> {
    values: &'a [(&'a str, u64)],
    orientation: BarOrientation,
    title: Option<&'a str>,
    color: Color,
}

impl<'a> BarChart<'a> {
    /// Create a chart from `(label, value)` pairs.
    pub const fn new(values: &'a [(&'a str, u64)]) -> Self {
        Self {
            values,
            orientation: BarOrientation::Vertical,
            title: None,
            color: Color::Cyan,
        }
    }

    /// Choose vertical columns or horizontal bars.
    pub const fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Add a title to the chart frame.
    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the bar color.
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Render the chart into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let bars = self
            .values
            .iter()
            .map(|(label, value)| {
                RatatuiBar::with_label(*label, *value).style(Style::default().fg(self.color))
            })
            .collect::<Vec<_>>();
        let max = self
            .values
            .iter()
            .map(|(_, value)| *value)
            .max()
            .unwrap_or(1)
            .max(1);
        match self.orientation {
            BarOrientation::Vertical => {
                let mut chart = RatatuiBarChart::new(bars).bar_width(3).bar_gap(1).max(max);
                if let Some(title) = self.title {
                    chart = chart.block(Block::bordered().title(title));
                }
                frame.render_widget(chart, area);
            }
            BarOrientation::Horizontal => {
                let mut chart = RatatuiBarChart::horizontal(bars).max(max);
                if let Some(title) = self.title {
                    chart = chart.block(Block::bordered().title(title));
                }
                frame.render_widget(chart, area);
            }
        }
    }
}
