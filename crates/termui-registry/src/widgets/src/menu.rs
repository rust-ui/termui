use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

/// A bordered menu with a caller-selected row.
#[must_use]
pub struct Menu<'a> {
    title: &'a str,
    items: &'a [&'a str],
    selected: usize,
}

impl<'a> Menu<'a> {
    pub fn new(title: &'a str, items: &'a [&'a str], selected: usize) -> Self {
        Self {
            title,
            items,
            selected,
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let style = if index == self.selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(37, 99, 235))
                } else {
                    Style::default()
                };
                Line::from(Span::styled(format!(" {} ", item), style))
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(lines).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(self.title),
            ),
            area,
        );
    }
}
