use ratatui::layout::{Constraint, Layout};
use termui_renderer::render_frame;
use termui_widgets::text_input::TextInput;

pub(super) fn render() -> Vec<String> {
    render_frame(8, |frame| {
        let [focused, gap, inactive] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .areas(frame.area());
        let _ = gap;

        TextInput::new("Email")
            .value("ada@example.com")
            .focused(true)
            .render(frame, focused);
        TextInput::new("Workspace")
            .placeholder("Choose a name")
            .render(frame, inactive);
    })
}
