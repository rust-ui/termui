#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc};

    use ratzilla::{
        event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
        ratatui::{
            layout::{Alignment, Position, Rect},
            style::{Color, Style},
            widgets::{Block, BorderType, Paragraph, Wrap},
            Frame, Terminal,
        },
        DomBackend, WebRenderer,
    };
    use termui_widgets::text_input::TextInput;

    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    enum Field {
        #[default]
        Email,
        Workspace,
    }

    impl Field {
        fn next(self) -> Self {
            match self {
                Self::Email => Self::Workspace,
                Self::Workspace => Self::Email,
            }
        }
    }

    struct App {
        email: String,
        workspace: String,
        active: Field,
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                email: "ada@example.com".to_owned(),
                workspace: String::new(),
                active: Field::Email,
            }
        }
    }

    impl App {
        fn active_value_mut(&mut self) -> &mut String {
            match self.active {
                Field::Email => &mut self.email,
                Field::Workspace => &mut self.workspace,
            }
        }
    }

    #[derive(Clone, Copy, Default)]
    struct Areas {
        email: Rect,
        workspace: Rect,
    }

    fn render(frame: &mut Frame, app: &App, areas: &mut Areas) {
        let area = frame.area();
        let outer = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Color::Rgb(63, 63, 70));
        let content = outer.inner(area);
        frame.render_widget(outer, area);

        let field_width = content.width.min(48);
        let field_x = content.x + content.width.saturating_sub(field_width) / 2;
        let stack_height = content.height.min(9);
        let stack_y = content.y + content.height.saturating_sub(stack_height) / 2;
        areas.email = Rect::new(field_x, stack_y, field_width, 3);
        areas.workspace = Rect::new(field_x, stack_y.saturating_add(4), field_width, 3);

        TextInput::new("Email")
            .value(&app.email)
            .placeholder("ada@example.com")
            .focused(app.active == Field::Email)
            .render(frame, areas.email);
        TextInput::new("Workspace")
            .value(&app.workspace)
            .placeholder("Choose a name")
            .focused(app.active == Field::Workspace)
            .render(frame, areas.workspace);

        let hint_y = stack_y.saturating_add(7);
        let hint_height = content.height.saturating_sub(hint_y.saturating_sub(content.y));
        let hint = Rect::new(content.x, hint_y, content.width, hint_height.min(2));
        frame.render_widget(
            Paragraph::new(
                "Click a field to focus · Type to edit\nTab switches fields · Backspace deletes",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray))
            .wrap(Wrap { trim: true }),
            hint,
        );
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: Areas) {
        if event.kind != MouseEventKind::ButtonDown(MouseButton::Left) {
            return;
        }

        let point = Position::new(event.col, event.row);
        if areas.email.contains(point) {
            app.active = Field::Email;
        } else if areas.workspace.contains(point) {
            app.active = Field::Workspace;
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
                    KeyCode::Tab => app.active = app.active.next(),
                    KeyCode::Backspace => {
                        app.active_value_mut().pop();
                    }
                    KeyCode::Char('a') if event.ctrl => app.active_value_mut().clear(),
                    KeyCode::Char(character) if !event.ctrl && !event.alt => {
                        app.active_value_mut().push(character);
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
