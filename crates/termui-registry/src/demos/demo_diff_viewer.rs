use termui_renderer::render_frame;
use termui_widgets::diff_viewer::DiffViewer;

pub(super) fn render() -> Vec<String> {
    let lines = ["- old value", "+ new value", "  unchanged"];
    render_frame(3, |frame| {
        DiffViewer::new(&lines).render(frame, frame.area())
    })
}
