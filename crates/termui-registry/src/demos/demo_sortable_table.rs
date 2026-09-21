use termui_renderer::render_frame;
use termui_widgets::sortable_table::SortableTable;

pub(super) fn render() -> Vec<String> {
    let headers = ["Name", "Size"];
    let rows: [&[&str]; 2] = [&["README.md", "2 KB"], &["src/", "4 KB"]];
    render_frame(3, |frame| {
        SortableTable::new(&headers, &rows, 0, true).render(frame, frame.area());
    })
}
