use termui_renderer::render_frame;
use termui_widgets::scrollbar::Scrollbar;

pub(super) fn render() -> Vec<String> {
    render_frame(5, |frame| {
        Scrollbar::new(20, 5, 5).render(frame, frame.area());
    })
}
