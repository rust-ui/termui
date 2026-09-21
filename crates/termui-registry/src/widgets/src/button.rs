use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A compact terminal button. The parent app owns focus and activation.
#[must_use]
pub struct Button<'a> {
    label: &'a str,
    style: Style,
    focused: bool,
    disabled: bool,
    variant: ButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
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

/// End shape for a one-row terminal button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonShape {
    /// Curved pill ends (`( label )`).
    #[default]
    Rounded,
    /// Square bracket ends (`[ label ]`).
    Square,
}

impl ButtonSize {
    const fn padding(self) -> usize {
        match self {
            Self::Sm => 1,
            Self::Default => 2,
            Self::Lg => 3,
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
            shape: ButtonShape::default(),
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

    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
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
        let (left, right) = match self.shape {
            ButtonShape::Rounded => ('(', ')'),
            ButtonShape::Square => ('[', ']'),
        };
        let padding = " ".repeat(self.size.padding());
        let end_style = Style {
            fg: Some(style.bg.or(style.fg).unwrap_or(Color::Reset)),
            add_modifier: style.add_modifier,
            sub_modifier: style.sub_modifier,
            ..Style::default()
        };
        Line::from(vec![
            Span::styled(left.to_string(), end_style),
            Span::styled(format!("{padding}{}{padding}", self.label), style),
            Span::styled(right.to_string(), end_style),
        ])
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
