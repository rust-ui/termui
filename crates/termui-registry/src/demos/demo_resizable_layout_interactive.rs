#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout, Position, Rect},
            style::{Color, Style},
            widgets::{Block, BorderType, Paragraph},
        },
    };
    use termui_widgets::resizable_layout::ResizableLayout;

    #[derive(Clone, Copy)]
    enum InteractionState {
        Idle,
        Active,
        Dragging,
    }

    impl InteractionState {
        fn from_app(app: &App) -> Self {
            if app.dragging {
                Self::Dragging
            } else if app.active {
                Self::Active
            } else {
                Self::Idle
            }
        }

        fn label(self) -> &'static str {
            match self {
                Self::Idle => "Idle",
                Self::Active => "Active",
                Self::Dragging => "Dragging",
            }
        }
    }

    #[derive(Default)]
    struct App {
        first_percent: u16,
        active: bool,
        dragging: bool,
    }

    impl App {
        fn new() -> Self {
            Self {
                first_percent: 32,
                active: false,
                dragging: false,
            }
        }

        fn set_from_pointer(&mut self, column: u16, area: Rect) {
            let panels_width = area.width.saturating_sub(u16::from(area.width >= 3));
            if panels_width == 0 {
                return;
            }
            let offset = column.saturating_sub(area.x).min(panels_width);
            self.first_percent =
                (u32::from(offset) * 100 / u32::from(panels_width)).clamp(15, 85) as u16;
        }
    }

    #[derive(Clone, Copy, Default)]
    struct Areas {
        panels: Rect,
        handle: Rect,
    }

    fn render(frame: &mut Frame, app: &App, hit_areas: &mut Areas) {
        let [status_area, panels_area, hint_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        frame.render_widget(
            Paragraph::new(format!(
                "Sidebar {:>2}%  │  Main panel {:>2}%  │  {state}",
                app.first_percent,
                100 - app.first_percent,
                state = InteractionState::from_app(app).label(),
            ))
            .style(Style::default().fg(Color::White)),
            status_area,
        );
        frame.render_widget(
            Paragraph::new("Drag divider · Tab activates · ← / → resize")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            hint_area,
        );

        hit_areas.panels = panels_area;
        let [sidebar, handle, main] =
            ResizableLayout::new(app.first_percent).areas_with_handle(panels_area);
        hit_areas.handle = handle;

        frame.render_widget(
            Paragraph::new("Files\nProjects").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Sidebar"),
            ),
            sidebar,
        );
        frame.render_widget(
            Paragraph::new(if app.active || app.dragging {
                "┃"
            } else {
                "│"
            })
            .alignment(Alignment::Center)
            .style(Style::default().fg(if app.active || app.dragging {
                Color::White
            } else {
                Color::DarkGray
            })),
            handle,
        );
        frame.render_widget(
            Paragraph::new("Content\nDetails").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Main panel"),
            ),
            main,
        );
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: Areas) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => {
                app.active = app.dragging || areas.handle.contains(point);
                if app.dragging {
                    app.set_from_pointer(event.col, areas.panels);
                }
            }
            MouseEventKind::Exited => {
                app.active = false;
                app.dragging = false;
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.handle.contains(point) => {
                app.active = true;
                app.dragging = true;
                app.set_from_pointer(event.col, areas.panels);
            }
            MouseEventKind::ButtonUp(MouseButton::Left) => {
                app.dragging = false;
                app.active = areas.handle.contains(point);
            }
            _ => {}
        }
    }

    pub fn run() -> io::Result<()> {
        let app = Rc::new(RefCell::new(App::new()));
        let areas = Rc::new(RefCell::new(Areas::default()));
        let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;

        terminal.on_key_event({
            let app = Rc::clone(&app);
            move |event| {
                let mut app = app.borrow_mut();
                match event.code {
                    KeyCode::Tab => app.active = !app.active,
                    KeyCode::Esc => app.active = false,
                    KeyCode::Left if app.active => {
                        app.first_percent = app.first_percent.saturating_sub(2).max(15);
                    }
                    KeyCode::Right if app.active => {
                        app.first_percent = (app.first_percent + 2).min(85);
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
            let app = app.borrow();
            let mut areas = areas.borrow_mut();
            render(frame, &app, &mut areas);
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
