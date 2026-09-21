use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// A vertical scroll indicator for caller-owned content and scroll state.
#[must_use]
pub struct Scrollbar {
    content_length: usize,
    viewport_length: usize,
    position: usize,
}

impl Scrollbar {
    pub fn new(content_length: usize, viewport_length: usize, position: usize) -> Self {
        Self {
            content_length,
            viewport_length,
            position,
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let height = area.height as usize;
        let thumb_height = if self.content_length == 0 {
            height
        } else {
            (height * self.viewport_length / self.content_length).clamp(1, height.max(1))
        };
        let track = height.saturating_sub(thumb_height);
        let scroll_range = self.content_length.saturating_sub(self.viewport_length);
        let thumb_top = if scroll_range == 0 {
            0
        } else {
            self.position.min(scroll_range) * track / scroll_range
        };
        let lines = (0..height)
            .map(|row| {
                let (symbol, color) = if (thumb_top..thumb_top + thumb_height).contains(&row) {
                    ("█", Color::Gray)
                } else {
                    ("░", Color::DarkGray)
                };
                Line::from(Span::styled(symbol, Style::default().fg(color)))
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), area);
    }
}
