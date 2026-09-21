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
    use termui_widgets::form_field::FormField;

    let value = Rc::new(RefCell::new(String::from("ada@example.com")));
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
            Layout::vertical([Constraint::Length(5), Constraint::Length(1)]).areas(frame.area());
        let value = value.borrow();
        let valid = value.contains('@') && value.contains('.');
        let widget = if valid {
            FormField::new("Email", &value).validation("Email looks valid")
        } else {
            FormField::new("Email", &value)
        };
        widget.render(frame, field);
        frame.render_widget(
            Paragraph::new("Type to edit · Backspace deletes").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
