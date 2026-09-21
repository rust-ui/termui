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
    use std::{cell::RefCell, io, rc::Rc, time::Duration};
    use termui_widgets::toast::Toast;

    let toast = Rc::new(RefCell::new(Some(Toast::error(io::Error::other(
        "Build completed successfully",
    )))));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let toast = Rc::clone(&toast);
        move |event| {
            if event.code == KeyCode::Enter {
                let mut slot = toast.borrow_mut();
                if slot.as_ref().is_some_and(|item| !item.is_closed()) {
                    if let Some(item) = slot.as_mut() {
                        item.close();
                    }
                } else {
                    *slot = Some(Toast::error(io::Error::other(
                        "Build completed successfully",
                    )));
                }
            }
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [content, hint] =
            Layout::vertical([Constraint::Min(2), Constraint::Length(1)]).areas(frame.area());
        let mut slot = toast.borrow_mut();
        if let Some(item) = slot.as_mut() {
            item.tick(Duration::from_millis(16));
            if !item.is_closed() || item.is_animating() {
                item.render(frame, content);
            } else {
                frame.render_widget(
                    Paragraph::new("Notification dismissed").alignment(Alignment::Center),
                    content,
                );
            }
        }
        frame.render_widget(
            Paragraph::new("Enter dismisses or shows the toast").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
