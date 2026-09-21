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
    use termui_widgets::toggle_switch::ToggleSwitch;

    let enabled = Rc::new(Cell::new(true));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let enabled = Rc::clone(&enabled);
        move |event| {
            if matches!(event.code, KeyCode::Enter | KeyCode::Char(' ')) {
                enabled.set(!enabled.get());
            }
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [control, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        frame.render_widget(
            Paragraph::new(
                ToggleSwitch::new("Notifications")
                    .checked(enabled.get())
                    .line(),
            ),
            control,
        );
        frame.render_widget(
            Paragraph::new("Press Space to switch")
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
