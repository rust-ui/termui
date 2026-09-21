use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{BarChart, BarOrientation};

pub(super) fn render() -> Vec<String> {
    let values = [("Mon", 5), ("Tue", 8), ("Wed", 4), ("Thu", 10), ("Fri", 7)];
    render_frame(14, |frame| {
        BarChart::new(&values)
            .orientation(BarOrientation::Vertical)
            .color(Color::Cyan)
            .title("Weekly activity")
            .render(frame, frame.area());
    })
}
