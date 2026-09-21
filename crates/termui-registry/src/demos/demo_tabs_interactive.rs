#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use std::{cell::Cell, rc::Rc};

    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            widgets::{Block, Paragraph},
        },
    };
    use termui_widgets::tabs::Tabs;

    let selected = Rc::new(Cell::new(0usize));
    let labels = ["Overview", "Activity", "Settings"];
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Left => selected.set((selected.get() + 2) % 3),
            KeyCode::Right => selected.set((selected.get() + 1) % 3),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [tabs, content, hint] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        let index = selected.get();
        frame.render_widget(Paragraph::new(Tabs::new(&labels, index).line()), tabs);
        let body = [
            "3 environments · 12 members",
            "Build completed · 2 minutes ago",
            "Notifications enabled",
        ][index];
        frame.render_widget(
            Paragraph::new(body).block(Block::bordered().title(labels[index])),
            content,
        );
        frame.render_widget(
            Paragraph::new("← / → switch tab").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
