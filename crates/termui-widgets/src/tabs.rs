use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// A static tab row. The parent app owns navigation and selected state.
#[must_use]
pub struct Tabs<'a> {
    labels: &'a [&'a str],
    selected: usize,
}

impl<'a> Tabs<'a> {
    pub fn new(labels: &'a [&'a str], selected: usize) -> Self {
        Self { labels, selected }
    }

    pub fn line(&self) -> Line<'static> {
        let spans = self
            .labels
            .iter()
            .enumerate()
            .flat_map(|(index, label)| {
                let (text, style) = if index == self.selected {
                    (
                        format!("[{}]", label),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    )
                } else {
                    ((*label).to_owned(), Style::default().fg(Color::Gray))
                };
                let mut row = Vec::with_capacity(2);
                if index > 0 {
                    row.push(Span::raw("  "));
                }
                row.push(Span::styled(text, style));
                row
            })
            .collect::<Vec<_>>();
        Line::from(spans)
    }
}
