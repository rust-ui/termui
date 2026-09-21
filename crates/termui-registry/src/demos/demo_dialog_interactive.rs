#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc, time::Duration};

    use ratzilla::{
        event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
        ratatui::{
            layout::{Constraint, Layout, Position, Rect},
            style::{Color, Style},
            widgets::{Paragraph, Wrap},
            Frame, Terminal,
        },
        DomBackend, WebRenderer,
    };
    use termui_widgets::{
        button::{Button, ButtonVariant},
        dialog::{
            Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
            DialogTitle, DialogTrigger,
        },
    };

    #[derive(Default)]
    struct App {
        dialog: Dialog,
        hover: Option<Position>,
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct Areas {
        trigger: Rect,
        cancel: Rect,
        confirm: Rect,
    }

    fn render(frame: &mut Frame, app: &mut App, hit_areas: &mut Areas) {
        app.dialog.tick(Duration::from_millis(16));

        let area = frame.area();
        let trigger_width = 22.min(area.width);
        hit_areas.trigger = Rect::new(
            area.x + area.width.saturating_sub(trigger_width) / 2,
            area.y + area.height / 2,
            trigger_width,
            1,
        );
        DialogTrigger::new("Delete project")
            .focused(
                app.hover
                    .is_some_and(|point| hit_areas.trigger.contains(point)),
            )
            .render(frame, hit_areas.trigger);

        let Some(dialog_areas) = DialogContent::new()
            .width(Constraint::Length(44))
            .height(Constraint::Length(12))
            .render(frame, area, &mut app.dialog)
        else {
            hit_areas.cancel = Rect::default();
            hit_areas.confirm = Rect::default();
            return;
        };

        DialogHeader::new(DialogTitle::new("Delete project?"))
            .description(DialogDescription::new("This action cannot be undone."))
            .render(frame, dialog_areas.header);
        frame.render_widget(
            Paragraph::new("All project files and settings will be removed.")
                .style(Style::default().fg(Color::Gray))
                .wrap(Wrap { trim: true }),
            dialog_areas.body,
        );

        let actions = DialogFooter::new().render(frame, dialog_areas.footer);
        let [_spacer, cancel, confirm] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(12),
            Constraint::Length(12),
        ])
        .areas(actions);
        hit_areas.cancel = cancel;
        hit_areas.confirm = confirm;
        DialogClose::new("Cancel")
            .focused(
                app.hover
                    .is_some_and(|point| hit_areas.cancel.contains(point)),
            )
            .render(frame, cancel);
        Button::new("Delete")
            .variant(ButtonVariant::Destructive)
            .focused(
                app.hover
                    .is_some_and(|point| hit_areas.confirm.contains(point)),
            )
            .render(frame, confirm);
    }

    fn handle_mouse(event: MouseEvent, app: &mut App, areas: Areas) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => app.hover = Some(point),
            MouseEventKind::Exited => app.hover = None,
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.cancel.contains(point) => {
                DialogClose::new("Cancel").activate(&mut app.dialog);
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.confirm.contains(point) => {
                app.dialog.close();
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if areas.trigger.contains(point) => {
                if app.dialog.is_closed() {
                    DialogTrigger::new("Delete project").activate(&mut app.dialog);
                }
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if !app.dialog.is_closed() => {
                app.dialog.close_on_outside_click(point);
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
                    KeyCode::Esc => app.dialog.close(),
                    KeyCode::Char(' ') | KeyCode::Enter if app.dialog.is_closed() => {
                        app.dialog.open();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => app.dialog.close(),
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
