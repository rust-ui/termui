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
    use termui_widgets::diff_viewer::DiffViewer;

    let changes = [
        "@@ config.rs @@",
        "- timeout = 15",
        "+ timeout = 30",
        "  retries = 3",
        "- mode = \"fast\"",
        "+ mode = \"safe\"",
        "  logging = true",
    ];
    let offset = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let offset = Rc::clone(&offset);
        move |event| match event.code {
            KeyCode::Up => offset.set(offset.get().saturating_sub(1)),
            KeyCode::Down => offset.set((offset.get() + 1).min(changes.len() - 1)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [diff, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        DiffViewer::new(&changes[offset.get()..]).render(frame, diff);
        frame.render_widget(Paragraph::new("↑ / ↓ inspect changes"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
