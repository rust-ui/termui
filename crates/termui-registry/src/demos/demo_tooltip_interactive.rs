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
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::tooltip::Tooltip;

    let visible = Rc::new(Cell::new(false));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let visible = Rc::clone(&visible);
        move |event| {
            if event.code == KeyCode::Enter {
                visible.set(!visible.get());
            }
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [target, hint] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).areas(frame.area());
        if visible.get() {
            Tooltip::new("Deploys the latest successful build").render(frame, target);
        } else {
            frame.render_widget(
                Paragraph::new("Deploy build").alignment(Alignment::Center),
                target,
            );
        }
        frame.render_widget(
            Paragraph::new("Press Enter to show or hide the tooltip").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
