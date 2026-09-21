use termui_renderer::render_frame;
use termui_widgets::tooltip::Tooltip;

pub(super) fn render() -> Vec<String> {
    render_frame(3, |frame| {
        Tooltip::new("Press Enter to continue").render(frame, frame.area());
    })
}
