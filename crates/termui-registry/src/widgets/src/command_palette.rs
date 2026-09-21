use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

/// A filtered command list with a selected result.
#[must_use]
pub struct CommandPalette<'a> {
    query: &'a str,
    commands: &'a [&'a str],
    selected: usize,
}

impl<'a> CommandPalette<'a> {
    pub fn new(query: &'a str, commands: &'a [&'a str], selected: usize) -> Self {
        Self {
            query,
            commands,
            selected,
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Commands");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let [query_area, commands_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Cyan)),
                Span::raw(self.query),
            ])),
            query_area,
        );
        let lines = self
            .commands
            .iter()
            .enumerate()
            .map(|(index, command)| {
                let style = if index == self.selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(37, 99, 235))
                } else {
                    Style::default()
                };
                Line::from(Span::styled(format!("  {command}"), style))
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), commands_area);
    }
}
