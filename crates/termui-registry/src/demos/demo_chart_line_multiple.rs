use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{ChartSeries, line_chart::LineChart};

pub(super) fn render() -> Vec<String> {
    let desktop = [(0.0, 4.0), (1.0, 7.0), (2.0, 5.0), (3.0, 9.0), (4.0, 8.0)];
    let mobile = [(0.0, 2.0), (1.0, 3.0), (2.0, 6.0), (3.0, 5.0), (4.0, 7.0)];
    render_frame(14, |frame| {
        LineChart::new()
            .series(ChartSeries::new("Desktop", &desktop).color(Color::Cyan))
            .series(ChartSeries::new("Mobile", &mobile).color(Color::Yellow))
            .title("Sessions")
            .render(frame, frame.area());
    })
}
