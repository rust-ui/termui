use termui_renderer::render_frame;
use termui_widgets::multi_select_list::MultiSelectList;

pub(super) fn render() -> Vec<String> {
    let items = [("Rust", true), ("Go", false), ("TypeScript", true)];
    render_frame(3, |frame| {
        MultiSelectList::new(&items).render(frame, frame.area());
    })
}
