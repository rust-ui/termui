use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// A compact table of text rows.
#[must_use]
pub struct Table<'a> {
    headers: &'a [&'a str],
    rows: &'a [&'a [&'a str]],
}

impl<'a> Table<'a> {
    pub fn new(headers: &'a [&'a str], rows: &'a [&'a [&'a str]]) -> Self {
        Self { headers, rows }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(table_lines(self.headers, self.rows, None)),
            area,
        );
    }
}

pub(crate) fn table_lines(
    headers: &[&str],
    rows: &[&[&str]],
    sort: Option<(usize, bool)>,
) -> Vec<Line<'static>> {
    let widths = (0..headers.len())
        .map(|column| {
            rows.iter()
                .filter_map(|row| row.get(column))
                .map(|value| value.len())
                .chain(std::iter::once(headers[column].len()))
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    let mut lines = Vec::with_capacity(rows.len() + 1);
    let header = headers
        .iter()
        .enumerate()
        .map(|(column, value)| {
            let (label, extra) = match sort {
                Some((sorted, ascending)) if sorted == column => {
                    (format!("{value} {}", if ascending { "↑" } else { "↓" }), 1)
                }
                _ => ((*value).to_owned(), 0),
            };
            format!("{label:<width$}", width = widths[column] + extra)
        })
        .collect::<Vec<_>>()
        .join("  ");
    lines.push(Line::from(Span::styled(
        header,
        Style::default().fg(Color::White),
    )));
    lines.extend(rows.iter().map(|row| {
        let text = widths
            .iter()
            .enumerate()
            .map(|(column, width)| format!("{:<width$}", row.get(column).copied().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("  ");
        Line::from(text)
    }));
    lines
}
