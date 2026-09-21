use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use ratzilla::event::KeyCode;
use ratzilla::ratatui::style::{Color, Style};
use ratzilla::ratatui::text::Line;
use ratzilla::{DomBackend, WebRenderer};

use termui_widgets::select_list::SelectList;

const ITEMS: [&str; 4] = ["Overview", "Installation", "Components", "Changelog"];

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let mut terminal = ratzilla::ratatui::Terminal::new(backend)?;

    let selected = Rc::new(RefCell::new(0usize));

    terminal.on_key_event({
        let selected = selected.clone();
        move |key_event| {
            let mut selected = selected.borrow_mut();
            match key_event.code {
                KeyCode::Down => *selected = (*selected + 1).min(ITEMS.len() - 1),
                KeyCode::Up => *selected = selected.saturating_sub(1),
                _ => {}
            }
        }
    })?;

    terminal.draw_web(move |frame| {
        let items: Vec<Line<'static>> = ITEMS.iter().map(|item| Line::from(*item)).collect();

        SelectList::new(&items, *selected.borrow())
            .selection_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .render(frame, frame.area());
    });

    Ok(())
}
