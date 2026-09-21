use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::progress::Progress;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        let [label, gauge] =
            Layout::horizontal([Constraint::Length(5), Constraint::Min(1)]).areas(frame.area());
        frame.render_widget(Paragraph::new("CPU"), label);
        Progress::new("", 80).render(frame, gauge);
    })
}
