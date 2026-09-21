#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc, time::Duration};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Flex, Layout, Position, Rect},
            style::{Color, Style},
            widgets::Paragraph,
        },
    };
    use termui_widgets::toast::{
        Toast, ToastClose, ToastContent, ToastDescription, ToastTitle, ToastTracker, ToastTrigger,
        ToastVariant,
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

    #[derive(Default)]
    struct App {
        toast: Toast,
        hover: Option<Position>,
        active_variant: usize,
        next_variant: usize,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Areas {
        triggers: [Rect; 5],
        close: Option<Rect>,
    }

    fn activate_variant(app: &mut App, index: usize) {
        app.active_variant = index;
        app.next_variant = (index + 1) % VARIANTS.len();
        ToastTrigger::new(trigger_label(index)).activate(&mut app.toast);
    }

    fn trigger_label(index: usize) -> &'static str {
        match index {
            0 => "Show default",
            1 => "Show success",
            2 => "Show info",
            3 => "Show warning",
            _ => "Show error",
        }
    }

    fn trigger_style(variant: ToastVariant) -> Style {
        match variant {
            ToastVariant::Default => Style::default().fg(Color::Black).bg(Color::White),
            ToastVariant::Success => Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(34, 197, 94)),
            ToastVariant::Info => Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(59, 130, 246)),
            ToastVariant::Warning => Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(245, 158, 11)),
            ToastVariant::Error => Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(239, 68, 68)),
        }
    }

    fn render(frame: &mut Frame, app: &mut App, areas: &mut Areas) {
        app.toast.tick(Duration::from_millis(16));

        let area = frame.area();
        let [content] = Layout::vertical([Constraint::Length(7)])
            .flex(Flex::Center)
            .areas(area);
        let [buttons, hint] =
            Layout::vertical([Constraint::Length(5), Constraint::Length(2)]).areas(content);
        for (index, (variant, _, _)) in VARIANTS.iter().enumerate() {
            let button_width = 14.min(buttons.width);
            areas.triggers[index] = Rect::new(
                buttons.x + buttons.width.saturating_sub(button_width) / 2,
                buttons.y + index as u16,
                button_width,
                1,
            );
            let label = trigger_label(index);
            ToastTrigger::new(label)
                .style(trigger_style(*variant))
                .focused(
                    app.hover
                        .is_some_and(|point| areas.triggers[index].contains(point)),
                )
                .render(frame, areas.triggers[index]);
        }
        frame.render_widget(
            Paragraph::new("Click a colored trigger or press 1–5 to show that toast.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            hint,
        );

        let (variant, title, description) = VARIANTS[app.active_variant];
        if let Some(toast_areas) =
            ToastContent::new()
                .variant(variant)
                .render(frame, area, &mut app.toast)
        {
            ToastTitle::new(title)
                .variant(variant)
                .render(frame, toast_areas.title);
            ToastDescription::new(description).render(frame, toast_areas.description);
            areas.close = toast_areas.close;
            if let Some(close) = toast_areas.close {
                ToastClose::new()
                    .focused(app.hover.is_some_and(|point| close.contains(point)))
                    .render(frame, close);
            }
            if let Some(tracker) = toast_areas.tracker {
                ToastTracker::new()
                    .variant(variant)
                    .render(frame, tracker, &app.toast);
            }
        } else {
            areas.close = None;
        }
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: Areas) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => app.hover = Some(point),
            MouseEventKind::Exited => app.hover = None,
            MouseEventKind::ButtonDown(MouseButton::Left)
                if areas.close.is_some_and(|area| area.contains(point)) =>
            {
                ToastClose::new().activate(&mut app.toast);
            }
            MouseEventKind::ButtonDown(MouseButton::Left) => {
                if let Some(index) = areas.triggers.iter().position(|area| area.contains(point)) {
                    activate_variant(app, index);
                } else {
                    app.toast.close_on_outside_click(point);
                }
            }
            _ => {}
        }
    }

    pub fn run() -> io::Result<()> {
        let app = Rc::new(RefCell::new(App::default()));
        let areas = Rc::new(RefCell::new(Areas::default()));
        let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;

        terminal.on_key_event({
            let app = Rc::clone(&app);
            move |event| {
                let mut app = app.borrow_mut();
                match event.code {
                    KeyCode::Esc => ToastClose::new().activate(&mut app.toast),
                    KeyCode::Char('1') => activate_variant(&mut app, 0),
                    KeyCode::Char('2') => activate_variant(&mut app, 1),
                    KeyCode::Char('3') => activate_variant(&mut app, 2),
                    KeyCode::Char('4') => activate_variant(&mut app, 3),
                    KeyCode::Char('5') => activate_variant(&mut app, 4),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        let index = app.next_variant;
                        activate_variant(&mut app, index);
                    }
                    _ => {}
                }
            }
        })?;

        terminal.on_mouse_event({
            let app = Rc::clone(&app);
            let areas = Rc::clone(&areas);
            move |event| {
                let mut app = app.borrow_mut();
                handle_mouse(event, &mut app, *areas.borrow());
            }
        })?;

        terminal.draw_web(move |frame| {
            let mut app = app.borrow_mut();
            let mut areas = areas.borrow_mut();
            render(frame, &mut app, &mut areas);
        });
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    wasm_app::run()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly. Use `pnpm demos:ratzilla:build` to build it.");
}
