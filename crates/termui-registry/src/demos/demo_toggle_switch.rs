use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::toggle_switch::ToggleSwitch;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        frame.render_widget(
            Paragraph::new(ToggleSwitch::new("Notifications").checked(true).line()),
            frame.area(),
        );
    })
}
