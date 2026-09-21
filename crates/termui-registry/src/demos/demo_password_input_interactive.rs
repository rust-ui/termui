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
    use std::{cell::RefCell, rc::Rc};
    use termui_widgets::password_input::PasswordInput;

    let value = Rc::new(RefCell::new(String::from("termui-secret")));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let value = Rc::clone(&value);
        move |event| match event.code {
            KeyCode::Backspace => {
                value.borrow_mut().pop();
            }
            KeyCode::Char(ch) if !event.ctrl && !event.alt => value.borrow_mut().push(ch),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [field, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        let value = value.borrow();
        PasswordInput::new("Password", &value).render(frame, field);
        frame.render_widget(
            Paragraph::new(format!(
                "{} characters · value stays masked",
                value.chars().count()
            ))
            .alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
