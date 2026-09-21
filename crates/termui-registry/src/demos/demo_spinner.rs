use termui_renderer::render_frame;
use termui_widgets::spinner::Spinner;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        Spinner::new("Working...")
            .frame(0)
            .render(frame, frame.area());
    })
}
