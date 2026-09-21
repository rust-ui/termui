use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::confirmation_prompt::ConfirmationPrompt;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        frame.render_widget(
            Paragraph::new(
                ConfirmationPrompt::new("Delete this file?")
                    .actions("Cancel", "Delete")
                    .line(),
            ),
            frame.area(),
        );
    })
}
