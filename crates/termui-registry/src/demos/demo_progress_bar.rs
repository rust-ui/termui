use ratatui::layout::{Constraint, Layout};
use termui_renderer::render_frame;
use termui_widgets::progress_bar::ProgressBar;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        let area = frame.area();
        let [label, bar] =
            Layout::horizontal([Constraint::Length(14), Constraint::Min(1)]).areas(area);
        frame.render_widget(ratatui::widgets::Paragraph::new("Downloading"), label);
        ProgressBar::new(60).render(frame, bar);
    })
}
