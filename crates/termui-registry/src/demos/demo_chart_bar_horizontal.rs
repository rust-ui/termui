use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{BarChart, BarOrientation};

pub(super) fn render() -> Vec<String> {
    let values = [("Rust", 48), ("TypeScript", 34), ("Python", 22), ("Go", 16)];
    render_frame(12, |frame| {
        BarChart::new(&values)
            .orientation(BarOrientation::Horizontal)
            .color(Color::Green)
            .title("Downloads by language")
            .render(frame, frame.area());
    })
}
