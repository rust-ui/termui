use termui_renderer::render_frame;
use termui_widgets::markdown_viewer::MarkdownViewer;

pub(super) fn render() -> Vec<String> {
    let markdown = "# Heading\nFormatted **text** and a list:\n- First item";
    render_frame(3, |frame| {
        MarkdownViewer::new(markdown).render(frame, frame.area());
    })
}
