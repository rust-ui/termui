use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::{ChartSeries, area_chart::AreaChart};

pub(super) fn render() -> Vec<String> {
    let desktop = [
        (0.0, 3.0),
        (1.0, 6.0),
        (2.0, 4.0),
        (3.0, 8.0),
        (4.0, 5.0),
        (5.0, 9.0),
    ];
    let mobile = [
        (0.0, 2.0),
        (1.0, 3.0),
        (2.0, 5.0),
        (3.0, 4.0),
        (4.0, 6.0),
        (5.0, 7.0),
    ];
    render_frame(14, |frame| {
        AreaChart::new()
            .series(ChartSeries::new("Desktop", &desktop).color(Color::Cyan))
            .series(ChartSeries::new("Mobile", &mobile).color(Color::Yellow))
            .title("Page views")
            .render(frame, frame.area());
    })
}
