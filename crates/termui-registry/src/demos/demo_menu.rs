use termui_renderer::render_frame;
use termui_widgets::menu::Menu;

pub(super) fn render() -> Vec<String> {
    let items = ["Open", "Save", "Quit"];
    render_frame(5, |frame| {
        Menu::new("File", &items, 0).render(frame, frame.area());
    })
}
