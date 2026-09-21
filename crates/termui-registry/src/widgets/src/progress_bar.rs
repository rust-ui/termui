use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Gauge;

/// A percentage progress bar. The parent app owns and updates progress.
#[must_use]
pub struct ProgressBar<'a> {
    label: &'a str,
    percent: u16,
}

impl<'a> ProgressBar<'a> {
    pub fn new(percent: u16) -> Self {
        Self {
            label: "",
            percent: percent.min(100),
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let label = if self.label.is_empty() {
            format!("{}%", self.percent)
        } else {
            format!("{}  {}%", self.label, self.percent)
        };
        frame.render_widget(
            Gauge::default()
                .percent(self.percent)
                .label(label)
                .gauge_style(Style::default().fg(Color::Rgb(59, 130, 246))),
            area,
        );
    }
}
