use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Padding, Paragraph};

/// A bordered hint rendered into an area chosen by the parent app.
#[must_use]
pub struct Tooltip<'a> {
    text: &'a str,
    style: Style,
}

impl<'a> Tooltip<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::default().fg(Color::Gray),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .padding(Padding::horizontal(1));
        frame.render_widget(
            Paragraph::new(self.text).style(self.style).block(block),
            area,
        );
    }
}
