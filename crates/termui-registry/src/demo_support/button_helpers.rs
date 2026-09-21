use ratatui::{layout::Rect, Frame};
use termui_widgets::button::{Button, ButtonSize, ButtonVariant};

use termui_renderer::PREVIEW_WIDTH;

pub(super) fn render_row(
    frame: &mut Frame<'_>,
    buttons: &[(&str, ButtonVariant, ButtonSize)],
    y: u16,
) {
    let mut x = 0;
    for (label, variant, size) in buttons {
        let button = Button::new(label).variant(*variant).size(*size);
        let width = button.line().width() as u16;
        if x + width > PREVIEW_WIDTH {
            break;
        }
        button.render(frame, Rect::new(x, y, width, 1));
        x += width + 3;
    }
}
