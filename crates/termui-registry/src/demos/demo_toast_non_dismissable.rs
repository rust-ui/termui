use std::time::Duration;

use termui_renderer::render_frame;
use termui_widgets::toast::{Toast, ToastContent, ToastDescription, ToastTitle, ToastTracker};

pub(super) fn render() -> Vec<String> {
    render_frame(6, |frame| {
        let mut toast = Toast::new().dismissible(false);
        toast.open();
        toast.tick(Duration::from_millis(160));
        if let Some(areas) = ToastContent::new().render(frame, frame.area(), &mut toast) {
            ToastTitle::new("Upload in progress").render(frame, areas.title);
            ToastDescription::new("This notification closes after the upload completes.")
                .render(frame, areas.description);
            if let Some(tracker) = areas.tracker {
                ToastTracker::new().render(frame, tracker, &toast);
            }
        }
    })
}
