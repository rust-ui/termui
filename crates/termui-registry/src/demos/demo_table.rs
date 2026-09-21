use termui_renderer::render_frame;
use termui_widgets::table::Table;

pub(super) fn render() -> Vec<String> {
    let headers = ["Name", "Status"];
    let rows: [&[&str]; 2] = [&["API", "Online"], &["Worker", "Offline"]];
    render_frame(3, |frame| {
        Table::new(&headers, &rows).render(frame, frame.area())
    })
}
