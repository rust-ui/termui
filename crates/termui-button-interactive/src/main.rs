#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc};

    use ratzilla::{
        event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
        ratatui::{
            layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
            style::{Color, Style},
            widgets::{Block, BorderType, Paragraph},
            Frame, Terminal,
        },
        DomBackend, WebRenderer,
    };
    use termui_widgets::button::{Button, ButtonVariant};

    #[derive(Default)]
    struct App {
        count: u64,
        hover: Option<Position>,
    }

    #[derive(Default, Clone, Copy)]
    struct ButtonAreas {
        increment: Rect,
        reset: Rect,
    }

    fn action_areas(area: Rect) -> ButtonAreas {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(24),
                Constraint::Length(3),
                Constraint::Length(12),
            ])
            .split(area);

        let row = area.y + area.height / 2;
        ButtonAreas {
            increment: Rect::new(columns[0].x, row, columns[0].width, 1),
            reset: Rect::new(columns[2].x, row, columns[2].width, 1),
        }
    }

    fn render(frame: &mut Frame, app: &App, hit_areas: &mut ButtonAreas) {
        let outer = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Color::Rgb(63, 63, 70));
        let content = outer.inner(frame.area());
        frame.render_widget(outer, frame.area());

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(content);

        frame.render_widget(
            Paragraph::new(format!("Count: {}", app.count))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(250, 250, 250))),
            rows[0],
        );

        *hit_areas = action_areas(rows[1]);
        Button::new("Increment +1")
            .variant(ButtonVariant::Default)
            .focused(
                app.hover
                    .is_some_and(|point| hit_areas.increment.contains(point)),
            )
            .render(frame, hit_areas.increment);
        Button::new("Reset")
            .variant(ButtonVariant::Secondary)
            .focused(
                app.hover
                    .is_some_and(|point| hit_areas.reset.contains(point)),
            )
            .render(frame, hit_areas.reset);

        frame.render_widget(
            Paragraph::new("Click Increment or press Space · R resets")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(113, 113, 122))),
            rows[2],
        );
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: ButtonAreas) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => app.hover = Some(point),
            MouseEventKind::Exited => app.hover = None,
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.increment.contains(point) => {
                app.count = app.count.saturating_add(1);
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.reset.contains(point) => {
                app.count = 0;
            }
            _ => {}
        }
    }

    pub fn run() -> io::Result<()> {
        let app = Rc::new(RefCell::new(App::default()));
        let areas = Rc::new(RefCell::new(ButtonAreas::default()));
        let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;

        terminal.on_key_event({
            let app = Rc::clone(&app);
            move |event| {
                let mut app = app.borrow_mut();
                match event.code {
                    KeyCode::Char(' ') | KeyCode::Enter => app.count = app.count.saturating_add(1),
                    KeyCode::Char('r') | KeyCode::Char('R') => app.count = 0,
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
    eprintln!("This demo runs in WebAssembly. Use `pnpm demos:ratzilla:serve` to launch it.");
}
