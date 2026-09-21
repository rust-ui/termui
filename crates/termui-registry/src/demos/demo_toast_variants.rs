use std::time::Duration;

use ratatui::{Frame, layout::Rect};
use termui_renderer::render_frame;
use termui_widgets::toast::{
    Toast, ToastClose, ToastContent, ToastDescription, ToastTitle, ToastTracker, ToastVariant,
};

const VARIANTS: [(ToastVariant, &str, &str); 5] = [
    (
        ToastVariant::Default,
        "Notification",
        "Your changes were saved.",
    ),
    (ToastVariant::Success, "Success", "Deployment completed."),
    (
        ToastVariant::Info,
        "Information",
        "A new version is available.",
    ),
    (ToastVariant::Warning, "Warning", "Storage is almost full."),
    (ToastVariant::Error, "Error", "Could not save your changes."),
];

fn render_toast(
    frame: &mut Frame<'_>,
    area: Rect,
    variant: ToastVariant,
    title: &str,
    description: &str,
) {
    let mut toast = Toast::new();
    toast.open();
    toast.tick(Duration::from_millis(160));
    if let Some(areas) = ToastContent::new()
        .variant(variant)
        .render(frame, area, &mut toast)
    {
        ToastTitle::new(title)
            .variant(variant)
            .render(frame, areas.title);
        ToastDescription::new(description).render(frame, areas.description);
        if let Some(close) = areas.close {
            ToastClose::new().render(frame, close);
        }
        if let Some(tracker) = areas.tracker {
            ToastTracker::new()
                .variant(variant)
                .render(frame, tracker, &toast);
        }
    }
}

pub(super) fn render() -> Vec<String> {
    render_frame(30, |frame| {
        for (index, (variant, title, description)) in VARIANTS.iter().enumerate() {
            let row = index as u16 * 6;
            let width = frame.area().width;
            render_toast(
                frame,
                Rect::new(0, row, width, 6),
                *variant,
                title,
                description,
            );
        }
    })
}
