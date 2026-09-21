//! Small, copy-pasteable Ratatui widgets. No runtime, no framework: each
//! module is a single struct with a builder API and a `render(frame, area)`
//! method, meant to be copied into your own app rather than pulled in as a
//! dependency.

pub mod button;
pub mod card;
pub mod color_picker;
pub mod dialog;
pub mod panel;
pub mod radio_card;
pub mod select_list;
pub mod text_input;
pub mod toast;
pub mod tui_overlay;
