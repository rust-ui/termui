use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

/// Single-line input presentation. The parent owns its value and key handling.
#[must_use]
pub struct TextInput<'a> {
    label: &'a str,
    value: &'a str,
    placeholder: &'a str,
    focused: bool,
}

impl<'a> TextInput<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            value: "",
            placeholder: "",
            focused: false,
        }
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let accent = Color::Rgb(59, 130, 246);
        let border_style = if self.focused {
            Style::default().fg(accent)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(self.label);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let content = if self.value.is_empty() {
            Span::styled(self.placeholder, Style::default().fg(Color::DarkGray))
        } else {
            Span::raw(self.value)
        };
        let mut spans = vec![content];
        if self.focused {
            spans.push(Span::styled(
                " ",
                Style::default().add_modifier(Modifier::REVERSED),
            ));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), inner);
    }
}
