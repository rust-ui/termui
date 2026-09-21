use termui_renderer::render_frame;
use termui_widgets::tree_view::TreeView;

pub(super) fn render() -> Vec<String> {
    let entries = [
        ("src/", 0, true),
        ("main.rs", 1, false),
        ("widgets/", 1, true),
        ("button.rs", 2, false),
    ];
    render_frame(4, |frame| {
        TreeView::new(&entries).render(frame, frame.area())
    })
}
