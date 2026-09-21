use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};

pub(super) fn render() -> Vec<String> {
    render_frame(9, |frame| {
        let area = frame.area();
        let areas = Card::new()
            .header_rows(2)
            .footer_rows(2)
            .render(frame, area);
        let header = CardHeader::new()
            .description_rows(1)
            .action_width(14)
            .layout(areas.header);

        CardTitle::new("Card Title").render(frame, header.title);
        CardDescription::new("Card Description").render(frame, header.description);
        CardAction::new(
            Paragraph::new("Card Action")
                .alignment(Alignment::Right)
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::UNDERLINED),
                ),
        )
        .render(frame, header.action);
        CardContent::new(Paragraph::new("Card Content")).render(frame, areas.content);
        CardFooter::new(Paragraph::new("Card Footer")).render(frame, areas.footer);
    })
}
