use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use termui_widgets::panel::Panel;

use termui_renderer::render_frame;

pub(super) fn render() -> Vec<String> {
    render_frame(7, |frame| {
        let area = frame.area();
        let content = Paragraph::new("Workers online: 4\nBuild finished · 3.2s").block(
            Panel::new()
                .title("Activity")
                .bottom_title("Esc Back")
                .border_style(Style::default().fg(Color::Cyan))
                .block(),
        );
        frame.render_widget(content, area);
    })
}
