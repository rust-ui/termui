//! Small, copy-pasteable Ratatui widgets. No runtime, no framework: each
//! module is a single struct with a builder API and a `render(frame, area)`
//! method, meant to be copied into your own app rather than pulled in as a
//! dependency.

pub mod key_bar;
pub mod panel;
pub mod select_list;
