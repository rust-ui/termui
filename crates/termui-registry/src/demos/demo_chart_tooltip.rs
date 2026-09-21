use ratatui::style::Color;
use termui_renderer::render_frame;
use termui_widgets::chart::chart_tooltip::{ChartTooltip, TooltipEntry};

pub(super) fn render() -> Vec<String> {
    let entries = [
        TooltipEntry::new("Desktop", "1,284", Color::Cyan),
        TooltipEntry::new("Mobile", "946", Color::Yellow),
    ];
    render_frame(6, |frame| {
        ChartTooltip::new("Apr 18", &entries).render(frame, frame.area());
    })
}
