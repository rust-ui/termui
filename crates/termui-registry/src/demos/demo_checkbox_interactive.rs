#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::{cell::Cell, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            style::{Color, Style},
            widgets::Paragraph,
        },
    };
    use termui_widgets::checkbox::Checkbox;

    let checked = Rc::new(Cell::new(false));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let checked = Rc::clone(&checked);
        move |event| {
            if matches!(event.code, KeyCode::Enter | KeyCode::Char(' ')) {
                checked.set(!checked.get());
            }
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [control, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        frame.render_widget(
            Paragraph::new(
                Checkbox::new("Enable notifications")
                    .checked(checked.get())
                    .line(),
            ),
            control,
        );
        frame.render_widget(
            Paragraph::new("Press Space to toggle")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            hint,
        );
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
