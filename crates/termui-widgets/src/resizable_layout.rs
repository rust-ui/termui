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

    /// Splits an area into a first panel, a one-cell handle, and a second panel.
    pub fn areas_with_handle(&self, area: Rect) -> [Rect; 3] {
        let handle_width = u16::from(area.width >= 3);
        let panels_width = area.width.saturating_sub(handle_width);

        if panels_width < 2 {
            return [
                Rect::new(area.x, area.y, panels_width, area.height),
                Rect::new(
                    area.x.saturating_add(panels_width),
                    area.y,
                    handle_width,
                    area.height,
                ),
                Rect::new(
                    area.x
                        .saturating_add(panels_width)
                        .saturating_add(handle_width),
                    area.y,
                    0,
                    area.height,
                ),
            ];
        }

        let first_width = (u32::from(panels_width) * u32::from(self.first_percent) / 100)
            .clamp(1, u32::from(panels_width - 1)) as u16;
        [
            Rect::new(area.x, area.y, first_width, area.height),
            Rect::new(
                area.x.saturating_add(first_width),
                area.y,
                handle_width,
                area.height,
            ),
            Rect::new(
                area.x
                    .saturating_add(first_width)
                    .saturating_add(handle_width),
                area.y,
                panels_width - first_width,
                area.height,
            ),
        ]
    }
}
