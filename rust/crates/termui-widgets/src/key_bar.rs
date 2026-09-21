use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// Bottom-of-screen key hint bar, e.g. `q Quit  ↑↓ Move  ↵ Select`.
#[must_use]
pub struct KeyBar {
    hints: Vec<(&'static str, &'static str)>,
    key_style: Style,
    label_style: Style,
}

impl KeyBar {
    pub fn new(hints: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            hints,
            key_style: Style::default(),
            label_style: Style::default(),
        }
    }

    pub fn key_style(mut self, style: Style) -> Self {
        self.key_style = style;
        self
    }

    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = style;
        self
    }

    pub fn line(&self) -> Line<'static> {
        let mut spans = Vec::with_capacity(self.hints.len() * 3);
        for (index, (key, label)) in self.hints.iter().enumerate() {
            if index > 0 {
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled((*key).to_string(), self.key_style));
            spans.push(Span::raw(" "));
            spans.push(Span::styled((*label).to_string(), self.label_style));
        }
        Line::from(spans)
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
