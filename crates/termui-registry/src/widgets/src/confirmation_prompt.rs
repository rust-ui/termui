use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// A one-line confirmation question with caller-owned actions.
#[must_use]
pub struct ConfirmationPrompt<'a> {
    question: &'a str,
    cancel: &'a str,
    confirm: &'a str,
}

impl<'a> ConfirmationPrompt<'a> {
    pub fn new(question: &'a str) -> Self {
        Self {
            question,
            cancel: "Cancel",
            confirm: "Confirm",
        }
    }

    pub fn actions(mut self, cancel: &'a str, confirm: &'a str) -> Self {
        self.cancel = cancel;
        self.confirm = confirm;
        self
    }

    pub fn line(&self) -> Line<'a> {
        Line::from(vec![
            Span::raw(format!("{}  ", self.question)),
            Span::styled(
                format!("[ {} ]", self.cancel),
                Style::default().fg(Color::Gray),
            ),
            Span::raw("  "),
            Span::styled(
                format!("[ {} ]", self.confirm),
                Style::default().fg(Color::Rgb(96, 165, 250)),
            ),
        ])
    }
}
