use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

/// A compact file and folder list with a caller-selected path.
#[must_use]
pub struct FilePicker<'a> {
    entries: &'a [(&'a str, bool)],
    selected: usize,
}

impl<'a> FilePicker<'a> {
    pub fn new(entries: &'a [(&'a str, bool)], selected: usize) -> Self {
        Self { entries, selected }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, (name, is_directory))| {
                let marker = if *is_directory { "▾ " } else { "  " };
                let prefix = if index == self.selected { "> " } else { "  " };
                let style = if index == self.selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                Line::from(Span::styled(format!("{prefix}{marker}{name}"), style))
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(lines).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Files"),
            ),
            area,
        );
    }
}
