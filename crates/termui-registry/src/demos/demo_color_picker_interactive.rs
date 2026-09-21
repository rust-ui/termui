#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::{cell::Cell, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            widgets::Paragraph,
        },
    };
    use termui_widgets::color_picker::ColorPicker;

    let selected = Rc::new(Cell::new(6usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Left => selected.set(selected.get().saturating_sub(1)),
            KeyCode::Right => selected.set((selected.get() + 1).min(7)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [picker, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        ColorPicker::new(selected.get()).render(frame, picker);
        frame.render_widget(
            Paragraph::new("← / → choose accent color").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
