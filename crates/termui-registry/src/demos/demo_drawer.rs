use std::time::Duration;

use ratatui::widgets::Paragraph;
use termui_renderer::render_frame;
use termui_widgets::drawer::{Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerTitle};

pub(super) fn render() -> Vec<String> {
    render_frame(14, |frame| {
        let mut drawer = Drawer::new();
        drawer.open();
        drawer.tick(Duration::from_millis(180));

        if let Some(areas) = DrawerContent::new().render(frame, frame.area(), &mut drawer) {
            DrawerTitle::new("Profile").render(frame, areas.title);
            DrawerDescription::new("Manage your account details.").render(frame, areas.description);
            frame.render_widget(
                Paragraph::new("Alex Morgan\nalex@example.com\n\nPlan: Pro\nStatus: Active"),
                areas.body,
            );
            DrawerClose::new().render(frame, areas.close);
        }
    })
}
