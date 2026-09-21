use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

const FRAMES: [&str; 4] = ["◐", "◓", "◑", "◒"];

/// A single spinner frame. The parent app owns animation timing.
#[must_use]
pub struct Spinner<'a> {
    label: &'a str,
    frame: usize,
}

impl<'a> Spinner<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, frame: 0 }
    }

    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = frame;
        self
    }

    pub fn line(&self) -> Line<'a> {
        Line::from(vec![
            Span::styled(
                FRAMES[self.frame % FRAMES.len()],
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(format!(" {}", self.label)),
        ])
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(Paragraph::new(self.line()), area);
    }
}
