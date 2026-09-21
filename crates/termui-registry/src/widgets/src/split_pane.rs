use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};

/// A labeled two-column view with a caller-controlled divider position.
#[must_use]
pub struct SplitPane<'a> {
    left_title: &'a str,
    left_content: &'a str,
    right_title: &'a str,
    right_content: &'a str,
    left_percent: u16,
}

impl<'a> SplitPane<'a> {
    pub fn new(
        left_title: &'a str,
        left_content: &'a str,
        right_title: &'a str,
        right_content: &'a str,
        left_percent: u16,
    ) -> Self {
        Self {
            left_title,
            left_content,
            right_title,
            right_content,
            left_percent: left_percent.clamp(1, 99),
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let [left, divider, right] = Layout::horizontal([
            Constraint::Percentage(self.left_percent),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(area);
        frame.render_widget(
            Paragraph::new(self.left_content).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(self.left_title),
            ),
            left,
        );
        frame.render_widget(
            Paragraph::new("│").style(Style::default().fg(Color::DarkGray)),
            divider,
        );
        frame.render_widget(
            Paragraph::new(self.right_content).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(self.right_title),
            ),
            right,
        );
    }
}
