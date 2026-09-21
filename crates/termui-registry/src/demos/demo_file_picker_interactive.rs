#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Constraint, Layout},
            widgets::Paragraph,
        },
    };
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };
    use termui_widgets::file_picker::FilePicker;

    let entries = [
        ("src/", true),
        ("components/", true),
        ("main.rs", false),
        ("Cargo.toml", false),
    ];
    let selected = Rc::new(Cell::new(0usize));
    let path = Rc::new(RefCell::new(String::from("src/")));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        let path = Rc::clone(&path);
        move |event| match event.code {
            KeyCode::Up => selected.set((selected.get() + entries.len() - 1) % entries.len()),
            KeyCode::Down => selected.set((selected.get() + 1) % entries.len()),
            KeyCode::Enter => *path.borrow_mut() = entries[selected.get()].0.to_owned(),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [picker, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        FilePicker::new(&entries, selected.get()).render(frame, picker);
        frame.render_widget(
            Paragraph::new(format!("↑ / ↓ move · Enter open · {}", path.borrow())),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
