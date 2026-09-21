use termui_renderer::render_frame;
use termui_widgets::help_screen::HelpScreen;

pub(super) fn render() -> Vec<String> {
    let shortcuts = [("↑/↓", "Navigate"), ("Enter", "Select"), ("Esc", "Back")];
    render_frame(4, |frame| {
        HelpScreen::new("Keyboard shortcuts", &shortcuts).render(frame, frame.area());
    })
}
