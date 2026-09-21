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
        Toast, ToastContent, ToastDescription, ToastTitle, ToastTracker, ToastTrigger,
    };

    #[derive(Default)]
    struct App {
        toast: Toast,
        hover: Option<Position>,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Areas {
        trigger: Rect,
    }

    fn render(frame: &mut Frame, app: &mut App, areas: &mut Areas) {
        app.toast.tick(Duration::from_millis(16));

        let area = frame.area();
        let [content] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::Center)
            .areas(area);
        let [trigger, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(2)]).areas(content);
        let trigger_width = 14.min(trigger.width);
        areas.trigger = Rect::new(
            trigger.x + trigger.width.saturating_sub(trigger_width) / 2,
            trigger.y,
            trigger_width,
            trigger.height,
        );
        ToastTrigger::new("Show toast")
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .focused(app.hover.is_some_and(|point| areas.trigger.contains(point)))
            .render(frame, areas.trigger);
        frame.render_widget(
            Paragraph::new(
                "Click Show toast or press Enter. Closes automatically after five seconds.",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray)),
            hint,
        );

        if let Some(toast_areas) = ToastContent::new().render(frame, area, &mut app.toast) {
            ToastTitle::new("Upload in progress").render(frame, toast_areas.title);
            ToastDescription::new("This notification cannot be closed manually.")
                .render(frame, toast_areas.description);
            if let Some(tracker) = toast_areas.tracker {
                ToastTracker::new().render(frame, tracker, &app.toast);
            }
        }
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: Areas) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => app.hover = Some(point),
            MouseEventKind::Exited => app.hover = None,
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.trigger.contains(point) => {
                ToastTrigger::new("Show toast").activate(&mut app.toast);
            }
            _ => {}
        }
    }

    pub fn run() -> io::Result<()> {
        let app = Rc::new(RefCell::new(App {
            toast: Toast::new().dismissible(false),
            hover: None,
        }));
        let areas = Rc::new(RefCell::new(Areas::default()));
        let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;

        terminal.on_key_event({
            let app = Rc::clone(&app);
            move |event| {
                if matches!(event.code, KeyCode::Char(' ') | KeyCode::Enter) {
                    ToastTrigger::new("Show toast").activate(&mut app.borrow_mut().toast);
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
