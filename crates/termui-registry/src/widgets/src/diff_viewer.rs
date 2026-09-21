use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A compact line-oriented diff. Prefix each line with `+`, `-`, or a space.
#[must_use]
pub struct DiffViewer<'a> {
    lines: &'a [&'a str],
}

impl<'a> DiffViewer<'a> {
    pub fn new(lines: &'a [&'a str]) -> Self {
        Self { lines }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .lines
            .iter()
            .map(|text| {
                let (style, prefix) = match text.as_bytes().first() {
                    Some(b'+') => (Style::default().fg(Color::LightGreen), "+ "),
                    Some(b'-') => (Style::default().fg(Color::LightRed), "- "),
                    _ => (Style::default().fg(Color::Gray), "  "),
                };
                let body = text.trim_start_matches(['+', '-']).trim_start();
                Line::from(vec![Span::styled(prefix, style), Span::styled(body, style)])
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
