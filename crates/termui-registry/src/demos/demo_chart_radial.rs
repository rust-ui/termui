use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::radial_chart::RadialChart;

pub(super) fn render() -> Vec<String> {
    render_frame(14, |frame| {
        RadialChart::new("CPU", 72.0, 100.0)
            .color(Color::Green)
            .title("System load")
            .render(frame, frame.area());
    })
}
