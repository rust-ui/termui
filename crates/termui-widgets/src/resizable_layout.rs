use ratatui::layout::{Constraint, Layout, Rect};

/// Two horizontal regions with a caller-controlled width ratio.
#[must_use]
pub struct ResizableLayout {
    first_percent: u16,
}

impl ResizableLayout {
    pub fn new(first_percent: u16) -> Self {
        Self {
            first_percent: first_percent.clamp(1, 99),
        }
    }

    pub fn areas(&self, area: Rect) -> [Rect; 2] {
        Layout::horizontal([
            Constraint::Percentage(self.first_percent),
            Constraint::Percentage(100 - self.first_percent),
        ])
        .areas(area)
    }
}
