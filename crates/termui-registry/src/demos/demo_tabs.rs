use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::tabs::Tabs;

pub(super) fn render() -> Vec<String> {
    let labels = ["Overview", "Settings", "Logs"];
    render_frame(1, |frame| {
        frame.render_widget(Paragraph::new(Tabs::new(&labels, 0).line()), frame.area());
    })
}
