#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    use ratzilla::{
        DomBackend, WebRenderer,
        event::KeyCode,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            style::Color,
            widgets::Paragraph,
        },
    };
    use std::{cell::Cell, rc::Rc};
    use termui_widgets::chart::chart_tooltip::{ChartTooltip, TooltipEntry};

    let point = Rc::new(Cell::new(0usize));
    let dates = ["Apr 16", "Apr 17", "Apr 18", "Apr 19"];
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let point = Rc::clone(&point);
        move |event| match event.code {
            KeyCode::Left => point.set((point.get() + dates.len() - 1) % dates.len()),
            KeyCode::Right => point.set((point.get() + 1) % dates.len()),
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [tooltip, hint] =
            Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(frame.area());
        let i = point.get();
        let entries = [
            TooltipEntry::new("Requests", ["1.2k", "1.8k", "2.4k", "1.6k"][i], Color::Cyan),
            TooltipEntry::new("Errors", ["12", "8", "14", "5"][i], Color::LightRed),
        ];
        ChartTooltip::new(dates[i], &entries).render(frame, tooltip);
        frame.render_widget(
            Paragraph::new("← / → inspect chart points").alignment(Alignment::Center),
            hint,
        );
    });
    Ok(())
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly.");
}
