use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

/// A bordered multi-line text presentation. The parent app owns editing state.
#[must_use]
pub struct TextArea<'a> {
    label: &'a str,
    value: &'a str,
}

impl<'a> TextArea<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.value).wrap(Wrap { trim: false }).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(self.label),
            ),
            area,
        );
    }
}
