use termui_renderer::render_frame;
use termui_widgets::search_input::SearchInput;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        SearchInput::new("")
            .placeholder("Find in files...")
            .render(frame, frame.area());
    })
}
