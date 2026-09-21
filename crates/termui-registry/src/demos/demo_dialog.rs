use ratatui::layout::Rect;
use termui_renderer::render_frame;
use termui_widgets::dialog::DialogTrigger;

pub(super) fn render() -> Vec<String> {
    render_frame(3, |frame| {
        let area = frame.area();
        let width = 20.min(area.width);
        let button = Rect::new(
            area.x + area.width.saturating_sub(width) / 2,
            area.y + area.height / 2,
            width,
            1,
        );
        DialogTrigger::new("Delete project").render(frame, button);
    })
}
