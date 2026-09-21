use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::gauge::Gauge;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        let [label, gauge] =
            Layout::horizontal([Constraint::Length(5), Constraint::Min(1)]).areas(frame.area());
        frame.render_widget(Paragraph::new("CPU"), label);
        Gauge::new("", 80).render(frame, gauge);
    })
}
