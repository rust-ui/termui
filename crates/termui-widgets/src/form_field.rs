use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

/// A labeled input presentation with caller-provided validation feedback.
#[must_use]
pub struct FormField<'a> {
    label: &'a str,
    value: &'a str,
    validation: Option<&'a str>,
}

impl<'a> FormField<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            validation: None,
        }
    }

    pub fn validation(mut self, message: &'a str) -> Self {
        self.validation = Some(message);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let [label, input, feedback] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);
        frame.render_widget(Paragraph::new(self.label), label);
        frame.render_widget(
            Paragraph::new(self.value).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            ),
            input,
        );
        if let Some(message) = self.validation {
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled("✓ ", Style::default().fg(Color::Green)),
                    Span::styled(message, Style::default().fg(Color::Green)),
                ])),
                feedback,
            );
        }
    }
}
