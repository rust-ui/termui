use termui_renderer::render_frame;
use termui_widgets::empty_state::EmptyState;

pub(super) fn render() -> Vec<String> {
    render_frame(7, |frame| {
        EmptyState::new("No projects yet")
            .description("Create a project to get started.")
            .action("Create project")
            .render(frame, frame.area());
    })
}
