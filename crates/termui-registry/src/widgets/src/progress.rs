use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Gauge as RatatuiGauge;

/// A labeled percentage progress indicator for a metric such as CPU or memory usage.
#[must_use]
pub struct Progress<'a> {
    label: &'a str,
    percent: u16,
}

impl<'a> Progress<'a> {
    pub fn new(label: &'a str, percent: u16) -> Self {
        Self {
            label,
            percent: percent.min(100),
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            RatatuiGauge::default()
                .percent(self.percent)
                .label(format!("{}  {}%", self.label, self.percent))
                .gauge_style(Style::default().fg(Color::Rgb(59, 130, 246))),
            area,
        );
    }
}
