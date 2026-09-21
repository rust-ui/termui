#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            widgets::Paragraph,
        },
    };
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };
    use termui_widgets::command_palette::CommandPalette;

    let commands = [
        "Create project",
        "Open settings",
        "Invite member",
        "Switch workspace",
    ];
    let query = Rc::new(RefCell::new(String::new()));
    let selected = Rc::new(Cell::new(0usize));
    let action = Rc::new(RefCell::new(String::from("Type to filter commands")));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let query = Rc::clone(&query);
        let selected = Rc::clone(&selected);
        let action = Rc::clone(&action);
        move |event| {
            let matches: Vec<&str> = commands
                .iter()
                .copied()
                .filter(|item| item.to_lowercase().contains(&query.borrow().to_lowercase()))
                .collect();
            match event.code {
                KeyCode::Up => selected.set(selected.get().saturating_sub(1)),
                KeyCode::Down => {
                    selected.set((selected.get() + 1).min(matches.len().saturating_sub(1)))
                }
                KeyCode::Enter => {
                    if let Some(item) = matches.get(selected.get()) {
                        *action.borrow_mut() = format!("Ran: {item}");
                    }
                }
                KeyCode::Backspace => {
                    query.borrow_mut().pop();
                    selected.set(0);
                }
                KeyCode::Char(ch) if !event.ctrl && !event.alt => {
                    query.borrow_mut().push(ch);
                    selected.set(0);
                }
                _ => {}
            }
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [palette, hint] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(1)]).areas(frame.area());
        let query = query.borrow();
        let matches: Vec<&str> = commands
            .iter()
            .copied()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .collect();
        CommandPalette::new(
            &query,
            &matches,
            selected.get().min(matches.len().saturating_sub(1)),
        )
        .render(frame, palette);
        frame.render_widget(
            Paragraph::new(action.borrow().as_str()).alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
