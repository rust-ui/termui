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
    use termui_widgets::drawer::{
        Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerTitle, DrawerTrigger,
    };

    #[derive(Default)]
    struct App {
        drawer: Drawer,
        hover: Option<Position>,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Areas {
        trigger: Rect,
        close: Option<Rect>,
    }

    fn render(frame: &mut Frame, app: &mut App, areas: &mut Areas) {
        app.drawer.tick(Duration::from_millis(16));

        let area = frame.area();
        let [content] = Layout::vertical([Constraint::Length(4)])
            .flex(Flex::Center)
            .areas(area);
        let [trigger, spacer, hint] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .areas(content);
        let trigger_width = 16.min(trigger.width);
        areas.trigger = Rect::new(
            trigger.x + trigger.width.saturating_sub(trigger_width) / 2,
            trigger.y,
            trigger_width,
            trigger.height,
        );
        DrawerTrigger::new("Open Trigger")
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .focused(app.hover.is_some_and(|point| areas.trigger.contains(point)))
            .render(frame, areas.trigger);
        frame.render_widget(Paragraph::new(" "), spacer);
        frame.render_widget(
            Paragraph::new("Opens the profile drawer\nClick outside or press Esc to close")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            hint,
        );

        if let Some(drawer_areas) = DrawerContent::new().render(frame, area, &mut app.drawer) {
            DrawerTitle::new("Profile").render(frame, drawer_areas.title);
            DrawerDescription::new("Manage your account details.")
                .render(frame, drawer_areas.description);
            frame.render_widget(
                Paragraph::new("Alex Morgan\nalex@example.com\n\nAccount\nPro plan · Active"),
                drawer_areas.body,
            );
            areas.close = Some(drawer_areas.close);
            DrawerClose::new()
                .focused(
                    app.hover
                        .is_some_and(|point| drawer_areas.close.contains(point)),
                )
                .render(frame, drawer_areas.close);
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
                DrawerClose::new().activate(&mut app.drawer);
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.trigger.contains(point) => {
                DrawerTrigger::new("Open Trigger").activate(&mut app.drawer);
            }
            MouseEventKind::ButtonDown(MouseButton::Left) => {
                app.drawer.close_on_outside_click(point);
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
                    KeyCode::Esc => DrawerClose::new().activate(&mut app.drawer),
                    KeyCode::Char(' ') | KeyCode::Enter if app.drawer.is_closed() => {
                        DrawerTrigger::new("Open Trigger").activate(&mut app.drawer);
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => app.drawer.close(),
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
