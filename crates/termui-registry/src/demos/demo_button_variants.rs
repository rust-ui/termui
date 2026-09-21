use ratatui::Frame;
use termui_widgets::button::{ButtonSize, ButtonVariant};

use super::button_helpers::render_row;
use termui_renderer::render_frame;

pub(super) fn render() -> Vec<String> {
    let rows = [
        [
            ("Default", ButtonVariant::Default, ButtonSize::Default),
            ("Secondary", ButtonVariant::Secondary, ButtonSize::Default),
            ("Delete", ButtonVariant::Destructive, ButtonSize::Default),
        ],
        [
            ("Outline", ButtonVariant::Outline, ButtonSize::Default),
            ("Ghost", ButtonVariant::Ghost, ButtonSize::Default),
            ("Link", ButtonVariant::Link, ButtonSize::Default),
        ],
    ];

    render_frame(4, |frame: &mut Frame<'_>| {
        for (row, buttons) in rows.iter().enumerate() {
            render_row(frame, buttons, row as u16 * 2);
        }
    })
}
