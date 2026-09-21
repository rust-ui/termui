use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// Centered guidance for a view with no content.
#[must_use]
pub struct EmptyState<'a> {
    title: &'a str,
    description: &'a str,
    action: Option<&'a str>,
}

impl<'a> EmptyState<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: "",
            action: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let rows = 2 + usize::from(self.action.is_some());
        let [_, content, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(rows as u16),
            Constraint::Min(0),
        ])
        .areas(area);
        let mut lines = vec![
            Line::styled(self.title, Style::default().fg(Color::White)),
            Line::styled(self.description, Style::default().fg(Color::Gray)),
        ];
        if let Some(action) = self.action {
            lines.push(Line::styled(
                format!("[ {action} ]"),
                Style::default().fg(Color::Rgb(96, 165, 250)),
            ));
        }
        frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), content);
    }
}
