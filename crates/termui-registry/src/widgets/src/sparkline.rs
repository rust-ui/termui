use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

const BARS: [&str; 8] = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/// A one-line chart for a short sequence of values.
#[must_use]
pub struct Sparkline<'a> {
    label: &'a str,
    values: &'a [u64],
}

impl<'a> Sparkline<'a> {
    pub fn new(label: &'a str, values: &'a [u64]) -> Self {
        Self { label, values }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let max = self.values.iter().copied().max().unwrap_or(0).max(1);
        let bars = self
            .values
            .iter()
            .map(|value| BARS[((*value * 7 / max) as usize).min(7)])
            .collect::<String>();
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::raw(format!("{}  ", self.label)),
                Span::styled(bars, Style::default().fg(Color::Cyan)),
            ])),
            area,
        );
    }
}
