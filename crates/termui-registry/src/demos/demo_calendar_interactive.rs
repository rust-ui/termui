#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::{cell::Cell, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Constraint, Layout},
            widgets::Paragraph,
        },
    };
    use termui_widgets::calendar::Calendar;

    let day = Rc::new(Cell::new(21u8));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let day = Rc::clone(&day);
        move |event| match event.code {
            KeyCode::Left => day.set(day.get().saturating_sub(1).max(1)),
            KeyCode::Right => day.set((day.get() + 1).min(30)),
            KeyCode::Up => day.set(day.get().saturating_sub(7).max(1)),
            KeyCode::Down => day.set((day.get() + 7).min(30)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [calendar, hint] =
            Layout::vertical([Constraint::Min(7), Constraint::Length(1)]).areas(frame.area());
        Calendar::new(2026, 9)
            .selected_day(day.get())
            .render(frame, calendar);
        frame.render_widget(Paragraph::new("← / → day · ↑ / ↓ week"), hint);
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
