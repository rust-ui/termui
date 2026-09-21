use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::calendar::Calendar;

/// A selected date and its month calendar.
#[must_use]
pub struct DatePicker {
    year: i32,
    month: u8,
    day: u8,
}

impl DatePicker {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let [selected, calendar] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
        frame.render_widget(
            Paragraph::new(format!(
                "Select date: {:04}-{:02}-{:02}",
                self.year, self.month, self.day
            )),
            selected,
        );
        Calendar::new(self.year, self.month)
            .selected_day(self.day)
            .render(frame, calendar);
    }
}
