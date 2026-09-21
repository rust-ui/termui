use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::status_bar::StatusBar;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        frame.render_widget(
            Paragraph::new(
                StatusBar::new("main")
                    .right("Ln 12, Col 4  │  UTF-8")
                    .line(),
            ),
            frame.area(),
        );
    })
}
