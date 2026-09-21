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
    use termui_widgets::table::Table;

    let headers = ["Project", "Status", "Builds"];
    let projects = ["Term/UI", "Rustify", "Docs", "Examples"];
    let statuses = ["Ready", "Building", "Ready", "Queued"];
    let builds = ["128", "84", "56", "19"];
    let selected = Rc::new(Cell::new(0usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let selected = Rc::clone(&selected);
        move |event| match event.code {
            KeyCode::Up => selected.set(selected.get().saturating_sub(1)),
            KeyCode::Down => selected.set((selected.get() + 1).min(3)),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [table, hint] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(frame.area());
        let names = (0..projects.len())
            .map(|index| {
                if index == selected.get() {
                    format!("> {}", projects[index])
                } else {
                    format!("  {}", projects[index])
                }
            })
            .collect::<Vec<_>>();
        let rows = (0..projects.len())
            .map(|index| [names[index].as_str(), statuses[index], builds[index]])
            .collect::<Vec<_>>();
        let rows = rows.iter().map(|row| &row[..]).collect::<Vec<_>>();
        Table::new(&headers, &rows).render(frame, table);
        frame.render_widget(
            Paragraph::new("↑ / ↓ select row").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
