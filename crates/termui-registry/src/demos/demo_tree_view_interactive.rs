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
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::tree_view::TreeView;

    let expanded = Rc::new(Cell::new(true));
    let selected = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let expanded = Rc::clone(&expanded);
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Up => selected.set(selected.get().saturating_sub(1)),
            KeyCode::Down => {
                selected.set((selected.get() + 1).min(if expanded.get() { 2 } else { 0 }))
            }
            KeyCode::Enter | KeyCode::Char(' ') if selected.get() == 0 => {
                expanded.set(!expanded.get())
            }
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [tree, hint] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(frame.area());
        let mut entries = vec![(
            if selected.get() == 0 {
                "> src/"
            } else {
                "  src/"
            },
            0,
            expanded.get(),
        )];
        if expanded.get() {
            entries.push((
                if selected.get() == 1 {
                    "> components/"
                } else {
                    "  components/"
                },
                1,
                false,
            ));
            entries.push((
                if selected.get() == 2 {
                    "> lib.rs"
                } else {
                    "  lib.rs"
                },
                1,
                false,
            ));
        }
        TreeView::new(&entries).render(frame, tree);
        frame.render_widget(
            Paragraph::new("↑ / ↓ move · Space expand/collapse").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
