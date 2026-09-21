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
    use termui_widgets::menu::Menu;

    let items = ["New file", "Open", "Save", "Close"];
    let selected = Rc::new(Cell::new(0usize));
    let action = Rc::new(RefCell::new(String::from("↑ / ↓ choose · Enter run")));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        let action = Rc::clone(&action);
        move |event| match event.code {
            KeyCode::Up => selected.set((selected.get() + items.len() - 1) % items.len()),
            KeyCode::Down => selected.set((selected.get() + 1) % items.len()),
            KeyCode::Enter => *action.borrow_mut() = format!("Selected: {}", items[selected.get()]),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [menu, hint] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(1)]).areas(frame.area());
        Menu::new("Workspace", &items, selected.get()).render(frame, menu);
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
