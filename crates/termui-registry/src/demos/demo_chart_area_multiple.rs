use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{ChartSeries, area_chart::AreaChart};

pub(super) fn render() -> Vec<String> {
    let api = [(0.0, 2.0), (1.0, 4.0), (2.0, 3.0), (3.0, 6.0), (4.0, 5.0)];
    let worker = [(0.0, 1.0), (1.0, 2.0), (2.0, 4.0), (3.0, 3.0), (4.0, 6.0)];
    render_frame(14, |frame| {
        AreaChart::new()
            .series(ChartSeries::new("API", &api).color(Color::Cyan))
            .series(ChartSeries::new("Worker", &worker).color(Color::Magenta))
            .title("Requests by service")
            .render(frame, frame.area());
    })
}
