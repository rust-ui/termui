use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::radar_chart::{RadarChart, RadarSeries, RadarValue};

pub(super) fn render() -> Vec<String> {
    let termui = [
        RadarValue {
            label: "Speed",
            value: 8.0,
        },
        RadarValue {
            label: "Safety",
            value: 9.0,
        },
        RadarValue {
            label: "DX",
            value: 7.0,
        },
        RadarValue {
            label: "Size",
            value: 8.0,
        },
        RadarValue {
            label: "Docs",
            value: 6.0,
        },
    ];
    let ratatui = [
        RadarValue {
            label: "Speed",
            value: 7.0,
        },
        RadarValue {
            label: "Safety",
            value: 8.0,
        },
        RadarValue {
            label: "DX",
            value: 8.0,
        },
        RadarValue {
            label: "Size",
            value: 7.0,
        },
        RadarValue {
            label: "Docs",
            value: 9.0,
        },
    ];
    render_frame(14, |frame| {
        RadarChart::new()
            .series(RadarSeries::new("Term/UI", &termui).color(Color::Cyan))
            .series(RadarSeries::new("Ratatui", &ratatui).color(Color::Yellow))
            .max(10.0)
            .title("Framework comparison")
            .render(frame, frame.area());
    })
}
