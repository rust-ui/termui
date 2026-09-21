use ratatui::Frame;
use termui_widgets::button::{ButtonSize, ButtonVariant};

use super::button_helpers::render_row;
use termui_renderer::render_frame;

pub(super) fn render() -> Vec<String> {
    let buttons = [
        ("Small", ButtonVariant::Default, ButtonSize::Sm),
        ("Default", ButtonVariant::Default, ButtonSize::Default),
        ("Large", ButtonVariant::Default, ButtonSize::Lg),
    ];

    render_frame(3, |frame: &mut Frame<'_>| render_row(frame, &buttons, 1))
}
