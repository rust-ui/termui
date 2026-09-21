use termui_renderer::render_frame;
use termui_widgets::calendar::Calendar;

pub(super) fn render() -> Vec<String> {
    render_frame(7, |frame| {
        Calendar::new(2026, 9)
            .selected_day(21)
            .render(frame, frame.area());
    })
}
