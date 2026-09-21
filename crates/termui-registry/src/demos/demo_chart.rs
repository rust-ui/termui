use termui_renderer::render_frame;
use termui_widgets::chart::Chart;

pub(super) fn render() -> Vec<String> {
    let values = [("A", 4), ("B", 7), ("C", 3), ("D", 8)];
    render_frame(4, |frame| Chart::new(&values).render(frame, frame.area()))
}
