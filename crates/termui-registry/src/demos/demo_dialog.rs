use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::dialog::Dialog;

pub(super) fn render() -> Vec<String> {
    render_frame(11, |frame| {
        let areas = Dialog::new("Delete project?")
            .size(34, 7)
            .footer_rows(1)
            .border_style(Style::default().fg(Color::Rgb(63, 63, 70)))
            .render(frame, frame.area());

        frame.render_widget(
            Paragraph::new("This action cannot be undone.").style(Style::default().fg(Color::Gray)),
            areas.body,
        );
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::raw("[ Cancel ]  "),
                Span::styled("[ Delete ]", Style::default().fg(Color::Rgb(239, 68, 68))),
            ]))
            .alignment(Alignment::Right),
            areas.footer,
        );
    })
}
