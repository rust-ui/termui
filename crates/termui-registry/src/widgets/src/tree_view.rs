use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A flattened tree. Each entry gives its label, depth, and expanded state.
#[must_use]
pub struct TreeView<'a> {
    entries: &'a [(&'a str, usize, bool)],
}

impl<'a> TreeView<'a> {
    pub fn new(entries: &'a [(&'a str, usize, bool)]) -> Self {
        Self { entries }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .entries
            .iter()
            .map(|(label, depth, expanded)| {
                let marker = if *expanded { "▾ " } else { "  " };
                Line::from(vec![
                    Span::raw("  ".repeat(*depth)),
                    Span::styled(marker, Style::default().fg(Color::Cyan)),
                    Span::raw(*label),
                ])
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
