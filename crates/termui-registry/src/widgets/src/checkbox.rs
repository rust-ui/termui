use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// A checkbox label. The parent app owns input and checked state.
#[must_use]
pub struct Checkbox<'a> {
    label: &'a str,
    checked: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            checked: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn line(&self) -> Line<'a> {
        let (mark, style) = if self.checked {
            ("[x]", Style::default().fg(Color::Cyan))
        } else {
            ("[ ]", Style::default().fg(Color::Gray))
        };
        Line::from(vec![
            Span::styled(mark, style),
            Span::raw(format!("  {}", self.label)),
        ])
    }
}
