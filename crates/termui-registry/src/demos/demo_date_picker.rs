use termui_renderer::render_frame;
use termui_widgets::date_picker::DatePicker;

pub(super) fn render() -> Vec<String> {
    render_frame(8, |frame| {
        DatePicker::new(2026, 9, 21).render(frame, frame.area());
    })
}
