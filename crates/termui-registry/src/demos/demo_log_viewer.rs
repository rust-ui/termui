use termui_renderer::render_frame;
use termui_widgets::log_viewer::LogViewer;

pub(super) fn render() -> Vec<String> {
    let entries = [
        ("10:42:01", "INFO", "Server started"),
        ("10:42:03", "WARN", "Retry scheduled"),
        ("10:42:05", "ERROR", "Request failed"),
    ];
    render_frame(3, |frame| {
        LogViewer::new(&entries).render(frame, frame.area())
    })
}
