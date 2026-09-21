use ratatui::layout::{Constraint, Layout, Rect};
use termui_renderer::render_frame;
use termui_widgets::radio_card::RadioCard;

pub(super) fn render() -> Vec<String> {
    render_frame(6, |frame| {
        let [standard, _gap, contrast] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(frame.area());
        let height = frame.area().height.min(5);

        RadioCard::new("Standard", "Balanced palette")
            .selected(true)
            .render(
                frame,
                Rect::new(standard.x, standard.y, standard.width, height),
            );
        RadioCard::new("High contrast", "Stronger borders").render(
            frame,
            Rect::new(contrast.x, contrast.y, contrast.width, height),
        );
    })
}
