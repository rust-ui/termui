use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A search prompt with caller-owned query and key handling.
#[must_use]
pub struct SearchInput<'a> {
    query: &'a str,
    placeholder: &'a str,
}

impl<'a> SearchInput<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            placeholder: "Search...",
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let text = if self.query.is_empty() {
            Span::styled(self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            Span::raw(self.query)
        };
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("/ ", Style::default().fg(Color::Cyan)),
                text,
            ])),
            area,
        );
    }
}
