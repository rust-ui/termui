use ratatui::style::{Color, Style};
use termui_widgets::key_bar::KeyBar;

use termui_renderer::render_frame;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        KeyBar::new(vec![("q", "Quit"), ("↑↓", "Move"), ("↵", "Select")])
            .key_style(Style::default().fg(Color::Yellow))
            .label_style(Style::default().fg(Color::White))
            .render(frame, frame.area());
    })
}
