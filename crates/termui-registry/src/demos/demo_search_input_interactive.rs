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
    use termui_widgets::search_input::SearchInput;

    let query = Rc::new(RefCell::new(String::new()));
    let items = ["Buttons", "Calendar", "Dialogs", "Tables", "Text inputs"];
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let query = Rc::clone(&query);
        move |event| match event.code {
            KeyCode::Backspace => {
                query.borrow_mut().pop();
            }
            KeyCode::Char(ch) if !event.ctrl && !event.alt => query.borrow_mut().push(ch),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [search, results] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(frame.area());
        let query = query.borrow();
        SearchInput::new(&query)
            .placeholder("Search docs...")
            .render(frame, search);
        let matches = items
            .iter()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .copied()
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(matches.join("\n")), results);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
