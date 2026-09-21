use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

/// One choice in a parent-owned radio group.
#[must_use]
pub struct RadioCard<'a> {
    title: &'a str,
    description: &'a str,
    selected: bool,
}

impl<'a> RadioCard<'a> {
    pub fn new(title: &'a str, description: &'a str) -> Self {
        Self {
            title,
            description,
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let accent = Color::Rgb(59, 130, 246);
        let border_style = if self.selected {
            Style::default().fg(accent)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let marker = if self.selected { "◉ " } else { "○ " };
        let title_style = if self.selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(Line::from(vec![
                Span::styled(marker, title_style),
                Span::styled(self.title, title_style),
            ]));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(
            Paragraph::new(self.description).style(Style::default().fg(Color::Gray)),
            inner,
        );
    }
}
