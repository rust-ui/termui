//! Small, copy-pasteable Ratatui widgets. No runtime, no framework: each
//! module is a single struct with a builder API and a `render(frame, area)`
//! method, meant to be copied into your own app rather than pulled in as a
//! dependency.

pub mod breadcrumbs;
pub mod button;
pub mod calendar;
pub mod card;
pub mod chart;
pub mod checkbox;
pub mod color_picker;
pub mod command_palette;
pub mod confirmation_prompt;
pub mod date_picker;
pub mod dialog;
pub mod diff_viewer;
pub mod empty_state;
pub mod file_picker;
pub mod form_field;
pub mod help_screen;
pub mod list;
pub mod loading_state;
pub mod log_viewer;
pub mod markdown_viewer;
pub mod menu;
pub mod multi_select_list;
pub mod panel;
pub mod password_input;
pub mod progress;
pub mod progress_bar;
pub mod radio_card;
pub mod resizable_layout;
pub mod scrollbar;
pub mod search_input;
pub mod select_list;
pub mod sortable_table;
pub mod sparkline;
pub mod spinner;
pub mod split_pane;
pub mod status_bar;
pub mod table;
pub mod tabs;
pub mod text_area;
pub mod text_input;
pub mod toast;
pub mod toggle_switch;
pub mod tooltip;
pub mod tree_view;
pub mod tui_overlay;
