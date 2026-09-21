use termui_renderer::render_frame;
use termui_widgets::sparkline::Sparkline;

pub(super) fn render() -> Vec<String> {
    let values = [1, 3, 2, 5, 4, 7, 6, 8];
    render_frame(1, |frame| {
        Sparkline::new("Activity", &values).render(frame, frame.area());
    })
}
