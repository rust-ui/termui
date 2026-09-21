use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A list of labels with caller-owned checked states.
#[must_use]
pub struct MultiSelectList<'a> {
    items: &'a [(&'a str, bool)],
}

impl<'a> MultiSelectList<'a> {
    pub fn new(items: &'a [(&'a str, bool)]) -> Self {
        Self { items }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .items
            .iter()
            .map(|(label, checked)| {
                let (mark, color) = if *checked {
                    ("[x]", Color::Cyan)
                } else {
                    ("[ ]", Color::Gray)
                };
                Line::from(vec![
                    Span::styled(mark, Style::default().fg(color)),
                    Span::raw(format!("  {label}")),
                ])
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
