use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::pie_chart::{PieChart, PieSlice};

pub(super) fn render() -> Vec<String> {
    let slices = [
        PieSlice::new("Desktop", 48.0, Color::Cyan),
        PieSlice::new("Mobile", 32.0, Color::Yellow),
        PieSlice::new("Tablet", 20.0, Color::Magenta),
    ];
    render_frame(14, |frame| {
        PieChart::new(&slices)
            .title("Traffic sources")
            .render(frame, frame.area());
    })
}
