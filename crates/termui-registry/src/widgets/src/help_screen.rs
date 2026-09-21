use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A compact keyboard shortcut reference.
#[must_use]
pub struct HelpScreen<'a> {
    title: &'a str,
    shortcuts: &'a [(&'a str, &'a str)],
}

impl<'a> HelpScreen<'a> {
    pub fn new(title: &'a str, shortcuts: &'a [(&'a str, &'a str)]) -> Self {
        Self { title, shortcuts }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let rows =
            Layout::vertical(vec![Constraint::Length(1); self.shortcuts.len() + 1]).split(area);
        frame.render_widget(
            Paragraph::new(Line::styled(self.title, Style::default().fg(Color::White))),
            rows[0],
        );
        for (index, (key, description)) in self.shortcuts.iter().enumerate() {
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(*key, Style::default().fg(Color::Cyan)),
                    Span::raw(format!("  {description}")),
                ])),
                rows[index + 1],
            );
        }
    }
}
