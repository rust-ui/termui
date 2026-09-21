#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Constraint, Layout},
        },
    };
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::radio_card::RadioCard;

    let selected = Rc::new(Cell::new(1usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Up => selected.set(selected.get().saturating_sub(1)),
            KeyCode::Down => selected.set((selected.get() + 1).min(2)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let areas = Layout::vertical([Constraint::Length(3); 3]).split(frame.area());
        RadioCard::new("Starter", "Personal projects")
            .selected(selected.get() == 0)
            .render(frame, areas[0]);
        RadioCard::new("Team", "Shared workspace")
            .selected(selected.get() == 1)
            .render(frame, areas[1]);
        RadioCard::new("Scale", "Larger organizations")
            .selected(selected.get() == 2)
            .render(frame, areas[2]);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
