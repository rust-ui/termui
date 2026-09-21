use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A compact loading message. The parent app owns any refresh or animation.
#[must_use]
pub struct LoadingState<'a> {
    message: &'a str,
}

impl<'a> LoadingState<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("… ", Style::default().fg(Color::Cyan)),
                Span::raw(self.message),
            ])),
            area,
        );
    }
}
