use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// A compact on/off switch label. The parent app owns interaction and state.
#[must_use]
pub struct ToggleSwitch<'a> {
    label: &'a str,
    checked: bool,
}

impl<'a> ToggleSwitch<'a> {
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
        let (state, style) = if self.checked {
            ("[ ON ]", Style::default().fg(Color::Green))
        } else {
            ("[ OFF ]", Style::default().fg(Color::Gray))
        };
        Line::from(vec![
            Span::raw(format!("{}  ", self.label)),
            Span::styled(state, style),
        ])
    }
}
