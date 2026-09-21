use ratatui::widgets::{Block, BorderType, Paragraph};
use termui_renderer::render_frame;
use termui_widgets::resizable_layout::ResizableLayout;

pub(super) fn render() -> Vec<String> {
    render_frame(4, |frame| {
        let [sidebar, main] = ResizableLayout::new(30).areas(frame.area());
        frame.render_widget(
            Paragraph::new("Files\nProjects").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Sidebar"),
            ),
            sidebar,
        );
        frame.render_widget(
            Paragraph::new("Content\nDetails").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Main panel"),
            ),
            main,
        );
    })
}
