use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{ChartSeries, line_chart::LineChart};

pub(super) fn render() -> Vec<String> {
    let points = [(0.0, 2.0), (1.0, 4.0), (2.0, 3.0), (3.0, 7.0), (4.0, 5.0)];
    render_frame(14, |frame| {
        LineChart::new()
            .series(ChartSeries::new("Deployments", &points).color(Color::Yellow))
            .step(true)
            .markers(true)
            .title("Deployments")
            .render(frame, frame.area());
    })
}
