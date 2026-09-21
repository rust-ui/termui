use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::radar_chart::{RadarChart, RadarSeries, RadarValue};

pub(super) fn render() -> Vec<String> {
    let values = [
        RadarValue {
            label: "Speed",
            value: 8.0,
        },
        RadarValue {
            label: "Safety",
            value: 7.0,
        },
        RadarValue {
            label: "DX",
            value: 9.0,
        },
        RadarValue {
            label: "Size",
            value: 6.0,
        },
        RadarValue {
            label: "Docs",
            value: 8.0,
        },
    ];
    render_frame(14, |frame| {
        RadarChart::new()
            .series(RadarSeries::new("Term/UI", &values).color(Color::Cyan))
            .max(10.0)
            .title("Crate profile")
            .render(frame, frame.area());
    })
}
