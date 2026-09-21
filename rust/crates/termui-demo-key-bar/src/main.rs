use std::io;

use ratzilla::ratatui::layout::{Constraint, Layout};
use ratzilla::ratatui::style::{Color, Modifier, Style};
use ratzilla::ratatui::widgets::Paragraph;
use ratzilla::{DomBackend, WebRenderer};

use termui_widgets::key_bar::KeyBar;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = ratzilla::ratatui::Terminal::new(backend)?;

    terminal.draw_web(move |frame| {
        let [top, bottom] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());

        frame.render_widget(
            Paragraph::new("Key hint bar, pinned to the bottom of the screen."),
            top,
        );

        KeyBar::new(vec![
            ("q", "Quit"),
            ("↑↓", "Move"),
            ("↵", "Select"),
        ])
        .key_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::Gray))
        .render(frame, bottom);
    });

    Ok(())
}
