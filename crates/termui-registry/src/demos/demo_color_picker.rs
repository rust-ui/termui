use termui_renderer::render_frame;
use termui_widgets::color_picker::ColorPicker;

pub(super) fn render() -> Vec<String> {
    render_frame(8, |frame| ColorPicker::new(6).render(frame, frame.area()))
}
