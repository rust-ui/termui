use std::io;

use ratzilla::ratatui::layout::Alignment;
use ratzilla::ratatui::style::{Color, Style};
use ratzilla::ratatui::text::Line;
use ratzilla::ratatui::widgets::Paragraph;
use ratzilla::{DomBackend, WebRenderer};

use termui_widgets::panel::Panel;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = ratzilla::ratatui::Terminal::new(backend)?;

    terminal.draw_web(move |frame| {
        let block = Panel::new()
            .title(Line::from("Panel").alignment(Alignment::Center))
            .border_style(Style::default().fg(Color::Cyan))
            .block();

        frame.render_widget(
            Paragraph::new("Rounded panel shell, copy-paste into your own app.")
                .alignment(Alignment::Center)
                .block(block),
            frame.area(),
        );
    });

    Ok(())
}
