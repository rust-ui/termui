use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::pie_chart::{PieChart, PieSlice};

pub(super) fn render() -> Vec<String> {
    let slices = [
        PieSlice::new("Core", 55.0, Color::Cyan),
        PieSlice::new("Docs", 30.0, Color::Yellow),
        PieSlice::new("Other", 15.0, Color::Magenta),
    ];
    render_frame(14, |frame| {
        PieChart::new(&slices)
            .donut(true)
            .title("Package downloads")
            .render(frame, frame.area());
    })
}
