use termui_renderer::render_frame;
use termui_widgets::split_pane::SplitPane;

pub(super) fn render() -> Vec<String> {
    render_frame(5, |frame| {
        SplitPane::new("Editor", "# Hello", "Preview", "Hello", 49).render(frame, frame.area());
    })
}
