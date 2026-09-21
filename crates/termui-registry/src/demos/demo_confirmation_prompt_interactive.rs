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
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };
    use termui_widgets::confirmation_prompt::ConfirmationPrompt;

    let choice = Rc::new(Cell::new(0usize));
    let result = Rc::new(RefCell::new(String::from("← / → choose · Enter confirm")));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let choice = Rc::clone(&choice);
        let result = Rc::clone(&result);
        move |event| match event.code {
            KeyCode::Left | KeyCode::Right => choice.set(1 - choice.get()),
            KeyCode::Enter => {
                *result.borrow_mut() = if choice.get() == 0 {
                    "Cancelled"
                } else {
                    "Deletion confirmed"
                }
                .to_owned()
            }
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [question, selected, hint] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        frame.render_widget(
            Paragraph::new(
                ConfirmationPrompt::new("Delete workspace?")
                    .actions("Cancel", "Delete")
                    .line(),
            ),
            question,
        );
        frame.render_widget(
            Paragraph::new(if choice.get() == 0 {
                "Choice: Cancel"
            } else {
                "Choice: Delete"
            })
            .alignment(Alignment::Center),
            selected,
        );
        frame.render_widget(
            Paragraph::new(result.borrow().as_str()).alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
