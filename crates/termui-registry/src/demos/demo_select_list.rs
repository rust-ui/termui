use ratatui::style::{Color, Style};
use ratatui::text::Line;
use termui_widgets::select_list::SelectList;

use termui_renderer::render_frame;

pub(super) fn render() -> Vec<String> {
    let items = vec![
        Line::from("Overview"),
        Line::from("Components"),
        Line::from("Themes"),
        Line::from("Settings"),
    ];

    render_frame(4, |frame| {
        SelectList::new(&items, 1)
            .selection_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(37, 99, 235)),
            )
            .render(frame, frame.area());
    })
}
