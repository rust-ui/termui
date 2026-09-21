use termui_renderer::render_frame;
use termui_widgets::file_picker::FilePicker;

pub(super) fn render() -> Vec<String> {
    let entries = [
        ("src/", true),
        ("main.rs", false),
        ("lib.rs", false),
        ("widgets/", true),
    ];
    render_frame(6, |frame| {
        FilePicker::new(&entries, 1).render(frame, frame.area())
    })
}
