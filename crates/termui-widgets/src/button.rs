use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A compact terminal button. The parent app owns focus and activation.
#[must_use]
pub struct Button<'a> {
    label: &'a str,
    style: Style,
    focused: bool,
    disabled: bool,
    variant: ButtonVariant,
    size: ButtonSize,
}

/// Shadcn-inspired terminal button variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// Horizontal padding for a terminal button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl ButtonSize {
    const fn padding(self) -> usize {
        match self {
            Self::Sm => 0,
            Self::Default => 1,
            Self::Lg => 2,
        }
    }
}

impl ButtonVariant {
    fn style(self) -> Style {
        let white = Color::Rgb(250, 250, 250);
        match self {
            Self::Default => Style::default().fg(white).bg(Color::Rgb(37, 99, 235)),
            Self::Secondary => Style::default().fg(white).bg(Color::Rgb(63, 63, 70)),
            Self::Destructive => Style::default().fg(white).bg(Color::Rgb(220, 38, 38)),
            Self::Outline => Style::default()
                .fg(Color::Rgb(228, 228, 231))
                .add_modifier(Modifier::UNDERLINED),
            Self::Ghost => Style::default().fg(Color::Rgb(228, 228, 231)),
            Self::Link => Style::default()
                .fg(Color::Rgb(96, 165, 250))
                .add_modifier(Modifier::UNDERLINED),
        }
    }
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            style: Style::default(),
            focused: false,
            disabled: false,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn line(&self) -> Line<'static> {
        let style = self.variant.style().patch(self.style);
        let style = if self.disabled {
            style.add_modifier(Modifier::DIM)
        } else if self.focused {
            style.add_modifier(Modifier::BOLD)
        } else {
            style
        };
        let content = match self.variant {
            ButtonVariant::Default | ButtonVariant::Secondary | ButtonVariant::Destructive => {
                self.label.to_string()
            }
            ButtonVariant::Outline => format!("[{}]", self.label),
            ButtonVariant::Ghost => self.label.to_string(),
            ButtonVariant::Link => self.label.to_string(),
        };
        let padding = " ".repeat(self.size.padding());
        let content = format!("{padding}{content}{padding}");
        Line::from(Span::styled(content, style))
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
