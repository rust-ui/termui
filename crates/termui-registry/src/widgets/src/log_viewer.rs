use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A compact log view with time, level, and message columns.
#[must_use]
pub struct LogViewer<'a> {
    entries: &'a [(&'a str, &'a str, &'a str)],
}

impl<'a> LogViewer<'a> {
    pub fn new(entries: &'a [(&'a str, &'a str, &'a str)]) -> Self {
        Self { entries }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .entries
            .iter()
            .map(|(time, level, message)| {
                let color = match *level {
                    "ERROR" => Color::LightRed,
                    "WARN" => Color::LightYellow,
                    _ => Color::LightGreen,
                };
                Line::from(vec![
                    Span::styled(*time, Style::default().fg(Color::DarkGray)),
                    Span::raw("  "),
                    Span::styled(format!("{level:<5}"), Style::default().fg(color)),
                    Span::raw(format!(" {message}")),
                ])
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
