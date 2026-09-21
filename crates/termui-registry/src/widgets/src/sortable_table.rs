use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

use crate::table::table_lines;

/// A table with a marked sort column. The parent app owns row order.
#[must_use]
pub struct SortableTable<'a> {
    headers: &'a [&'a str],
    rows: &'a [&'a [&'a str]],
    sort_column: usize,
    ascending: bool,
}

impl<'a> SortableTable<'a> {
    pub fn new(
        headers: &'a [&'a str],
        rows: &'a [&'a [&'a str]],
        sort_column: usize,
        ascending: bool,
    ) -> Self {
        Self {
            headers,
            rows,
            sort_column,
            ascending,
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(table_lines(
                self.headers,
                self.rows,
                Some((self.sort_column, self.ascending)),
            )),
            area,
        );
    }
}
