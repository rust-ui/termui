use termui_renderer::render_frame;
use termui_widgets::password_input::PasswordInput;

pub(super) fn render() -> Vec<String> {
    render_frame(3, |frame| {
        PasswordInput::new("Password", "secretpass").render(frame, frame.area());
    })
}
