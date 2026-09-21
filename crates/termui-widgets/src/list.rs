use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A simple list with an optional active row.
#[must_use]
pub struct List<'a> {
    items: &'a [&'a str],
    selected: Option<usize>,
}

impl<'a> List<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self {
            items,
            selected: None,
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                if self.selected == Some(index) {
                    Line::from(vec![
                        Span::styled("> ", Style::default().fg(Color::Cyan)),
                        Span::raw(*item),
                    ])
                } else {
                    Line::from(format!("  {item}"))
                }
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
