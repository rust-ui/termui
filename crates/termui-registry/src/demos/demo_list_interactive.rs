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
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::list::List;

    let items = ["Overview", "Activity", "Settings", "Members"];
    let selected = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Up => selected.set((selected.get() + items.len() - 1) % items.len()),
            KeyCode::Down => selected.set((selected.get() + 1) % items.len()),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [list, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        List::new(&items)
            .selected(selected.get())
            .render(frame, list);
        frame.render_widget(Paragraph::new("↑ / ↓ move selection"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
