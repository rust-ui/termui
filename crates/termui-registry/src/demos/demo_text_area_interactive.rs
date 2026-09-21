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
    use std::{cell::RefCell, rc::Rc};
    use termui_widgets::text_area::TextArea;

    let value = Rc::new(RefCell::new(String::from("Write a note...")));
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
        let [editor, hint] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(frame.area());
        TextArea::new("Message", &value.borrow()).render(frame, editor);
        frame.render_widget(Paragraph::new("Type to edit · Backspace deletes"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
