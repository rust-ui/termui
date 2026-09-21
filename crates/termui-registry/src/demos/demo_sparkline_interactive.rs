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
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };
    use termui_widgets::sparkline::Sparkline;

    let values = Rc::new(RefCell::new([2u64, 4, 3, 6, 5, 8, 7, 4]));
    let sample = Rc::new(Cell::new(7usize));
    let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;
    terminal.on_key_event({
        let values = Rc::clone(&values);
        let sample = Rc::clone(&sample);
        move |event| match event.code {
            KeyCode::Left => sample.set(sample.get().saturating_sub(1)),
            KeyCode::Right => sample.set((sample.get() + 1).min(7)),
            KeyCode::Up => {
                let index = sample.get();
                let mut data = values.borrow_mut();
                data[index] += 1;
            }
            KeyCode::Down => {
                let index = sample.get();
                let mut data = values.borrow_mut();
                data[index] = data[index].saturating_sub(1);
            }
            _ => {}
        }
    })?;
    terminal.draw_web(move |frame: &mut Frame| {
        let [chart, hint] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(frame.area());
        let values = values.borrow();
        Sparkline::new("Requests", &values[..]).render(frame, chart);
        frame.render_widget(
            Paragraph::new(format!(
                "Sample {}: {} · ← / → select · ↑ / ↓ adjust",
                sample.get() + 1,
                values[sample.get()]
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
