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
    use termui_widgets::breadcrumbs::Breadcrumbs;

    let path = ["Home", "Projects", "Term/UI"];
    let depth = Rc::new(Cell::new(2usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let depth = Rc::clone(&depth);
        move |event| match event.code {
            KeyCode::Left => depth.set(depth.get().saturating_sub(1)),
            KeyCode::Right => depth.set((depth.get() + 1).min(path.len() - 1)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [crumbs, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        let active = depth.get();
        frame.render_widget(
            Paragraph::new(Breadcrumbs::new(&path[..=active]).separator("›").line()),
            crumbs,
        );
        frame.render_widget(
            Paragraph::new(format!("Current page: {} · ← / → navigate", path[active]))
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
