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
    use termui_widgets::sortable_table::SortableTable;

    let headers = ["Project", "Status", "Builds"];
    let data: [[&str; 3]; 4] = [
        ["Term/UI", "Ready", "128"],
        ["Rustify", "Building", "84"],
        ["Docs", "Ready", "56"],
        ["Examples", "Queued", "19"],
    ];
    let column = Rc::new(Cell::new(0usize));
    let ascending = Rc::new(Cell::new(true));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let column = Rc::clone(&column);
        let ascending = Rc::clone(&ascending);
        move |event| match event.code {
            KeyCode::Left => column.set((column.get() + 2) % 3),
            KeyCode::Right => column.set((column.get() + 1) % 3),
            KeyCode::Enter => ascending.set(!ascending.get()),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [table, hint] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(frame.area());
        let col = column.get();
        let mut order = [0usize, 1, 2, 3];
        order.sort_by(|left, right| data[*left][col].cmp(data[*right][col]));
        if !ascending.get() {
            order.reverse();
        }
        let rows = order
            .iter()
            .map(|index| &data[*index][..])
            .collect::<Vec<_>>();
        SortableTable::new(&headers, &rows, col, ascending.get()).render(frame, table);
        frame.render_widget(
            Paragraph::new("← / → sort column · Enter reverse order").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
