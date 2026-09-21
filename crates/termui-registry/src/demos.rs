#[path = "demo_support/button_helpers.rs"]
mod button_helpers;
#[path = "demos/demo_breadcrumbs.rs"]
mod demo_breadcrumbs;
#[path = "demos/demo_button.rs"]
mod demo_button;
#[path = "demos/demo_button_sizes.rs"]
mod demo_button_sizes;
#[path = "demos/demo_button_variants.rs"]
mod demo_button_variants;
#[path = "demos/demo_calendar.rs"]
mod demo_calendar;
#[path = "demos/demo_card.rs"]
mod demo_card;
#[path = "demos/demo_chart.rs"]
mod demo_chart;
#[path = "demos/demo_checkbox.rs"]
mod demo_checkbox;
#[path = "demos/demo_color_picker.rs"]
mod demo_color_picker;
#[path = "demos/demo_command_palette.rs"]
mod demo_command_palette;
#[path = "demos/demo_confirmation_prompt.rs"]
mod demo_confirmation_prompt;
#[path = "demos/demo_date_picker.rs"]
mod demo_date_picker;
#[path = "demos/demo_dialog.rs"]
mod demo_dialog;
#[path = "demos/demo_diff_viewer.rs"]
mod demo_diff_viewer;
#[path = "demos/demo_empty_state.rs"]
mod demo_empty_state;
#[path = "demos/demo_file_picker.rs"]
mod demo_file_picker;
#[path = "demos/demo_form_field.rs"]
mod demo_form_field;
#[path = "demos/demo_gauge.rs"]
mod demo_gauge;
#[path = "demos/demo_help_screen.rs"]
mod demo_help_screen;
#[path = "demos/demo_list.rs"]
mod demo_list;
#[path = "demos/demo_loading_state.rs"]
mod demo_loading_state;
#[path = "demos/demo_log_viewer.rs"]
mod demo_log_viewer;
#[path = "demos/demo_markdown_viewer.rs"]
mod demo_markdown_viewer;
#[path = "demos/demo_menu.rs"]
mod demo_menu;
#[path = "demos/demo_multi_select_list.rs"]
mod demo_multi_select_list;
#[path = "demos/demo_panel.rs"]
mod demo_panel;
#[path = "demos/demo_password_input.rs"]
mod demo_password_input;
#[path = "demos/demo_progress_bar.rs"]
mod demo_progress_bar;
#[path = "demos/demo_radio_card.rs"]
mod demo_radio_card;
#[path = "demos/demo_resizable_layout.rs"]
mod demo_resizable_layout;
#[path = "demos/demo_scrollbar.rs"]
mod demo_scrollbar;
#[path = "demos/demo_search_input.rs"]
mod demo_search_input;
#[path = "demos/demo_select_list.rs"]
mod demo_select_list;
#[path = "demos/demo_sortable_table.rs"]
mod demo_sortable_table;
#[path = "demos/demo_sparkline.rs"]
mod demo_sparkline;
#[path = "demos/demo_spinner.rs"]
mod demo_spinner;
#[path = "demos/demo_split_pane.rs"]
mod demo_split_pane;
#[path = "demos/demo_status_bar.rs"]
mod demo_status_bar;
#[path = "demos/demo_table.rs"]
mod demo_table;
#[path = "demos/demo_tabs.rs"]
mod demo_tabs;
#[path = "demos/demo_text_area.rs"]
mod demo_text_area;
#[path = "demos/demo_text_input.rs"]
mod demo_text_input;
#[path = "demos/demo_toast.rs"]
mod demo_toast;
#[path = "demos/demo_toggle_switch.rs"]
mod demo_toggle_switch;
#[path = "demos/demo_tooltip.rs"]
mod demo_tooltip;
#[path = "demos/demo_tree_view.rs"]
mod demo_tree_view;

type DemoRenderer = fn() -> Vec<String>;

const DEMOS: &[(&str, DemoRenderer)] = &[
    ("rust/breadcrumbs", demo_breadcrumbs::render),
    ("rust/calendar", demo_calendar::render),
    ("rust/button", demo_button::render),
    ("rust/button-sizes", demo_button_sizes::render),
    ("rust/button-variants", demo_button_variants::render),
    ("rust/card", demo_card::render),
    ("rust/chart", demo_chart::render),
    ("rust/checkbox", demo_checkbox::render),
    ("rust/command-palette", demo_command_palette::render),
    ("rust/confirmation-prompt", demo_confirmation_prompt::render),
    ("rust/color-picker", demo_color_picker::render),
    ("rust/dialog", demo_dialog::render),
    ("rust/diff-viewer", demo_diff_viewer::render),
    ("rust/date-picker", demo_date_picker::render),
    ("rust/empty-state", demo_empty_state::render),
    ("rust/form-field", demo_form_field::render),
    ("rust/file-picker", demo_file_picker::render),
    ("rust/gauge", demo_gauge::render),
    ("rust/help-screen", demo_help_screen::render),
    ("rust/loading-state", demo_loading_state::render),
    ("rust/list", demo_list::render),
    ("rust/log-viewer", demo_log_viewer::render),
    ("rust/menu", demo_menu::render),
    ("rust/markdown-viewer", demo_markdown_viewer::render),
    ("rust/multi-select-list", demo_multi_select_list::render),
    ("rust/panel", demo_panel::render),
    ("rust/password-input", demo_password_input::render),
    ("rust/progress-bar", demo_progress_bar::render),
    ("rust/radio-card", demo_radio_card::render),
    ("rust/resizable-layout", demo_resizable_layout::render),
    ("rust/select-list", demo_select_list::render),
    ("rust/scrollbar", demo_scrollbar::render),
    ("rust/search-input", demo_search_input::render),
    ("rust/sparkline", demo_sparkline::render),
    ("rust/sortable-table", demo_sortable_table::render),
    ("rust/spinner", demo_spinner::render),
    ("rust/split-pane", demo_split_pane::render),
    ("rust/status-bar", demo_status_bar::render),
    ("rust/tabs", demo_tabs::render),
    ("rust/table", demo_table::render),
    ("rust/text-area", demo_text_area::render),
    ("rust/text-input", demo_text_input::render),
    ("rust/toast", demo_toast::render),
    ("rust/tooltip", demo_tooltip::render),
    ("rust/toggle-switch", demo_toggle_switch::render),
    ("rust/tree-view", demo_tree_view::render),
];

pub(super) fn names() -> impl Iterator<Item = &'static str> {
    DEMOS.iter().map(|(name, _)| *name)
}

pub(super) fn render(name: &str) -> Option<Vec<String>> {
    DEMOS
        .iter()
        .find(|(demo_name, _)| *demo_name == name)
        .map(|(_, render)| render())
}
