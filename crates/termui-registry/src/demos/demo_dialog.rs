use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};
use termui_renderer::render_frame;
use termui_widgets::dialog::DialogTrigger;

pub(super) fn render() -> Vec<String> {
    render_frame(7, |frame| {
        let area = frame.area();
        let shell = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Color::Rgb(63, 63, 70));
        let inner = shell.inner(area);
        frame.render_widget(shell, area);

        let [title, description, trigger, _] =
            Layout::vertical([Constraint::Length(1); 4]).areas(inner);
        frame.render_widget(
            Paragraph::new("Project settings")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::White)),
            title,
        );
        frame.render_widget(
            Paragraph::new("Review before deleting")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            description,
        );
        let button = ratatui::layout::Rect::new(
            inner.x + inner.width.saturating_sub(20) / 2,
            trigger.y,
            20.min(inner.width),
            1,
        );
        DialogTrigger::new("Delete project").render(frame, button);
    })
}
