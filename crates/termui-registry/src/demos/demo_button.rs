use ratatui::layout::Rect;
use termui_widgets::button::Button;

use termui_renderer::{PREVIEW_WIDTH, render_frame};

pub(super) fn render() -> Vec<String> {
    render_frame(3, |frame| {
        Button::new("Save changes").render(frame, Rect::new(0, 1, PREVIEW_WIDTH, 1));
    })
}
