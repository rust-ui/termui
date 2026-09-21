use termui_renderer::render_frame;
use termui_widgets::text_area::TextArea;

pub(super) fn render() -> Vec<String> {
    render_frame(4, |frame| {
        TextArea::new("Message", "Write a message...").render(frame, frame.area());
    })
}
