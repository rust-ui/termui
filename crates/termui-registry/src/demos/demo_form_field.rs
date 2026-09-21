use termui_renderer::render_frame;
use termui_widgets::form_field::FormField;

pub(super) fn render() -> Vec<String> {
    render_frame(5, |frame| {
        FormField::new("Email", "name@example.com")
            .validation("Valid email address")
            .render(frame, frame.area());
    })
}
