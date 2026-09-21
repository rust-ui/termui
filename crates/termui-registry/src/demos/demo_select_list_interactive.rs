#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Constraint, Layout},
            style::{Color, Style},
            text::Line,
            widgets::Paragraph,
        },
    };
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::select_list::SelectList;

    let items = ["Development", "Staging", "Production", "Preview"].map(Line::from);
    let item_count = items.len();
    let selected = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Up => selected.set((selected.get() + item_count - 1) % item_count),
            KeyCode::Down => selected.set((selected.get() + 1) % item_count),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [list, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        SelectList::new(&items, selected.get())
            .selection_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .render(frame, list);
        frame.render_widget(Paragraph::new("↑ / ↓ choose environment"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
