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
    use termui_widgets::progress_bar::ProgressBar;

    let percent = Rc::new(Cell::new(68u16));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let percent = Rc::clone(&percent);
        move |event| match event.code {
            KeyCode::Left | KeyCode::Down => percent.set(percent.get().saturating_sub(5)),
            KeyCode::Right | KeyCode::Up => percent.set((percent.get() + 5).min(100)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [bar, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        ProgressBar::new(percent.get())
            .label("Upload")
            .render(frame, bar);
        frame.render_widget(
            Paragraph::new("← / → adjust upload progress")
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
