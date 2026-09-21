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
    use termui_widgets::log_viewer::LogViewer;

    let entries = [
        ("09:41:02", "INFO", "Server started"),
        ("09:41:03", "INFO", "Loaded 18 routes"),
        ("09:41:09", "WARN", "Cache nearing capacity"),
        ("09:41:12", "INFO", "Request completed"),
        ("09:41:18", "ERROR", "Connection timed out"),
        ("09:41:20", "INFO", "Retry scheduled"),
        ("09:41:24", "INFO", "Connection restored"),
        ("09:41:25", "INFO", "Health check passed"),
    ];
    let offset = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let offset = Rc::clone(&offset);
        move |event| match event.code {
            KeyCode::Up => offset.set(offset.get().saturating_sub(1)),
            KeyCode::Down => offset.set((offset.get() + 1).min(7)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [logs, hint] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(1)]).areas(frame.area());
        LogViewer::new(&entries[offset.get().min(entries.len() - 1)..]).render(frame, logs);
        frame.render_widget(Paragraph::new("↑ / ↓ scroll logs"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
