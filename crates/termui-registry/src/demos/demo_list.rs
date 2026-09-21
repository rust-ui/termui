use termui_renderer::render_frame;
use termui_widgets::list::List;

pub(super) fn render() -> Vec<String> {
    let items = ["First item", "Second item", "Third item"];
    render_frame(3, |frame| {
        List::new(&items).selected(0).render(frame, frame.area());
    })
}
