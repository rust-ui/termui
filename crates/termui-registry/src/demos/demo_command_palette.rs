use termui_renderer::render_frame;
use termui_widgets::command_palette::CommandPalette;

pub(super) fn render() -> Vec<String> {
    let commands = ["Open file", "Toggle theme", "Quit"];
    render_frame(5, |frame| {
        CommandPalette::new("", &commands, 0).render(frame, frame.area());
    })
}
