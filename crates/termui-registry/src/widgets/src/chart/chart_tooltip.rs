use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

/// A labeled value displayed inside a chart tooltip.
#[derive(Clone, Copy, Debug)]
pub struct TooltipEntry<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub color: Color,
}

impl<'a> TooltipEntry<'a> {
    /// Create one colored label and formatted value row.
    pub const fn new(label: &'a str, value: &'a str, color: Color) -> Self {
        Self {
            label,
            value,
            color,
        }
    }
}

/// Composable chart tooltip panel for selection and hover states.
#[must_use]
pub struct ChartTooltip<'a> {
    title: &'a str,
    entries: &'a [TooltipEntry<'a>],
}

impl<'a> ChartTooltip<'a> {
    /// Create a tooltip with a heading and one or more series values.
    pub const fn new(title: &'a str, entries: &'a [TooltipEntry<'a>]) -> Self {
        Self { title, entries }
    }

    /// Render the tooltip into a Ratatui frame region.
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .entries
            .iter()
            .map(|entry| {
                Line::from(vec![
                    Span::styled("● ", Style::default().fg(entry.color)),
                    Span::raw(entry.label),
                    Span::raw("  "),
                    Span::styled(entry.value, Style::default().fg(entry.color).bold()),
                ])
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(lines).block(Block::bordered().title(self.title)),
            area,
        );
    }
}
