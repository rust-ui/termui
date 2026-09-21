#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::{cell::Cell, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            widgets::Paragraph,
        },
    };
    use termui_widgets::split_pane::SplitPane;

    let percent = Rc::new(Cell::new(34u16));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let percent = Rc::clone(&percent);
        move |event| match event.code {
            KeyCode::Left => percent.set(percent.get().saturating_sub(5).max(15)),
            KeyCode::Right => percent.set((percent.get() + 5).min(75)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [panes, hint] =
            Layout::vertical([Constraint::Min(4), Constraint::Length(1)]).areas(frame.area());
        SplitPane::new(
            "Files",
            "src/\nCargo.toml",
            "Editor",
            "fn main() {\n    println!(\"hello\");\n}",
            percent.get(),
        )
        .render(frame, panes);
        frame.render_widget(
            Paragraph::new(format!(
                "← / → resize divider · {}% / {}%",
                percent.get(),
                100 - percent.get()
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
