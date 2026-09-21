use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::checkbox::Checkbox;

pub(super) fn render() -> Vec<String> {
    let lines = vec![
        Checkbox::new("Unchecked").line(),
        Checkbox::new("Checked").checked(true).line(),
    ];
    render_frame(2, |frame| {
        frame.render_widget(Paragraph::new(lines), frame.area())
    })
}
