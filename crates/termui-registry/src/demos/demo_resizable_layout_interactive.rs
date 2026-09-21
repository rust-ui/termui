#[cfg(target_arch = "wasm32")]
#[path = "../hooks/use_resizable_layout.rs"]
mod use_resizable_layout;

#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use std::{cell::RefCell, io, rc::Rc};

    use super::use_resizable_layout::{ResizableLayoutState, use_resizable_layout};
    use ratzilla::{
        DomBackend, WebRenderer,
        ratatui::{
            Frame, Terminal,
            layout::{Alignment, Constraint, Layout},
            style::{Color, Style},
            widgets::{Block, BorderType, Paragraph},
        },
    };
    use termui_widgets::resizable_layout::ResizableLayout;

    fn render(frame: &mut Frame, state: &mut ResizableLayoutState) {
        let [status_area, panels_area, hint_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        let state_label: &'static str = state.interaction_state().into();

        frame.render_widget(
            Paragraph::new(format!(
                "Sidebar {:>2}%  │  Main panel {:>2}%  │  {state}",
                state.first_percent(),
                100 - state.first_percent(),
                state = state_label,
            ))
            .style(Style::default().fg(Color::White)),
            status_area,
        );
        frame.render_widget(
            Paragraph::new("Drag divider · Tab activates · ← / → resize")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray)),
            hint_area,
        );

        let [sidebar, handle, main] =
            ResizableLayout::new(state.first_percent()).areas_with_handle(panels_area);
        state.set_hit_areas(panels_area, handle);

        frame.render_widget(
            Paragraph::new("Files\nProjects").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Sidebar"),
            ),
            sidebar,
        );
        frame.render_widget(
            Paragraph::new(if state.is_highlighted() { "┃" } else { "│" })
                .alignment(Alignment::Center)
                .style(Style::default().fg(if state.is_highlighted() {
                    Color::White
                } else {
                    Color::DarkGray
                })),
            handle,
        );
        frame.render_widget(
            Paragraph::new("Content\nDetails").block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Main panel"),
            ),
            main,
        );
    }

    pub fn run() -> io::Result<()> {
        let state = Rc::new(RefCell::new(use_resizable_layout()));
        let mut terminal = Terminal::new(DomBackend::new_by_id("terminal")?)?;

        terminal.on_key_event({
            let state = Rc::clone(&state);
            move |event| {
                state.borrow_mut().handle_key(event.code);
            }
        })?;

        terminal.on_mouse_event({
            let state = Rc::clone(&state);
            move |event| {
                state.borrow_mut().handle_mouse(event);
            }
        })?;

        terminal.draw_web(move |frame| {
            render(frame, &mut state.borrow_mut());
        });
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn main() -> std::io::Result<()> {
    wasm_app::run()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This demo runs in WebAssembly. Use `pnpm demos:ratzilla:build` to build it.");
}
