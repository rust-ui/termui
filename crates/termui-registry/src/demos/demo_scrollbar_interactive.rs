#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Constraint, Direction, Layout},
            widgets::Paragraph,
        },
    };
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::scrollbar::Scrollbar;

    let position = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let position = Rc::clone(&position);
        move |event| match event.code {
            KeyCode::Up => position.set(position.get().saturating_sub(1)),
            KeyCode::Down => position.set((position.get() + 1).min(16)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [content, bar] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(2)])
            .areas(frame.area());
        let lines = (0..8)
            .map(|row| format!("Event {}", position.get() + row + 1))
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines.join("\n")), content);
        Scrollbar::new(24, 8, position.get()).render(frame, bar);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
