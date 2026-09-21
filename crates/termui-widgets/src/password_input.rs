use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

/// A masked password value. The parent app owns editing and secret storage.
#[must_use]
pub struct PasswordInput<'a> {
    label: &'a str,
    value: &'a str,
}

impl<'a> PasswordInput<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let masked = "•".repeat(self.value.chars().count());
        frame.render_widget(
            Paragraph::new(masked).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(self.label),
            ),
            area,
        );
    }
}
