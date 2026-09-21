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
    use termui_widgets::multi_select_list::MultiSelectList;

    let labels = ["Build", "Deploy", "Preview", "Notify"];
    let selected = Rc::new(Cell::new(0usize));
    let checked = Rc::new(RefCell::new([true, false, true, false]));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        let checked = Rc::clone(&checked);
        move |event| match event.code {
            KeyCode::Up => selected.set((selected.get() + labels.len() - 1) % labels.len()),
            KeyCode::Down => selected.set((selected.get() + 1) % labels.len()),
            KeyCode::Enter | KeyCode::Char(' ') => {
                let index = selected.get();
                let mut flags = checked.borrow_mut();
                flags[index] = !flags[index];
            }
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [list, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        let flags = *checked.borrow();
        let items = [
            (labels[0], flags[0]),
            (labels[1], flags[1]),
            (labels[2], flags[2]),
            (labels[3], flags[3]),
        ];
        MultiSelectList::new(&items).render(frame, list);
        frame.render_widget(
            Paragraph::new("↑ / ↓ move · Space toggle").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
