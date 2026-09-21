use termui_renderer::render_frame;
use termui_widgets::loading_state::LoadingState;

pub(super) fn render() -> Vec<String> {
    render_frame(1, |frame| {
        LoadingState::new("Loading data...").render(frame, frame.area());
    })
}
