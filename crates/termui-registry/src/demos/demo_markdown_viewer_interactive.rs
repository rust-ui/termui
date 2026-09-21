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
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::markdown_viewer::MarkdownViewer;

    let markdown = "# Release notes\n\nVersion **2.4.0** is ready.\n\n- Faster startup\n- Better keyboard navigation\n- Fixed terminal resizing\n\n## Upgrade\n\nRun the package manager to install the latest release.\n\n## Support\n\nRead the guide or contact the team.";
    let offset = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let offset = Rc::clone(&offset);
        move |event| match event.code {
            KeyCode::Up => offset.set(offset.get().saturating_sub(1)),
            KeyCode::Down => offset.set((offset.get() + 1).min(8)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [document, hint] =
            Layout::vertical([Constraint::Min(5), Constraint::Length(1)]).areas(frame.area());
        let lines = markdown
            .lines()
            .skip(offset.get())
            .collect::<Vec<_>>()
            .join("\n");
        MarkdownViewer::new(&lines).render(frame, document);
        frame.render_widget(Paragraph::new("↑ / ↓ scroll document"), hint);
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
