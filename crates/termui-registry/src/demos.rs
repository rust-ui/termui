#[path = "demo_support/button_helpers.rs"]
mod button_helpers;
#[path = "demos/demo_button.rs"]
mod demo_button;
#[path = "demos/demo_button_sizes.rs"]
mod demo_button_sizes;
#[path = "demos/demo_button_variants.rs"]
mod demo_button_variants;
#[path = "demos/demo_color_picker.rs"]
mod demo_color_picker;
#[path = "demos/demo_dialog.rs"]
mod demo_dialog;
#[path = "demos/demo_key_bar.rs"]
mod demo_key_bar;
#[path = "demos/demo_panel.rs"]
mod demo_panel;
#[path = "demos/demo_radio_card.rs"]
mod demo_radio_card;
#[path = "demos/demo_select_list.rs"]
mod demo_select_list;
#[path = "demos/demo_text_input.rs"]
mod demo_text_input;
#[path = "demos/demo_toast.rs"]
mod demo_toast;

type DemoRenderer = fn() -> Vec<String>;

const DEMOS: &[(&str, DemoRenderer)] = &[
    ("rust/button", demo_button::render),
    ("rust/button-sizes", demo_button_sizes::render),
    ("rust/button-variants", demo_button_variants::render),
    ("rust/color-picker", demo_color_picker::render),
    ("rust/dialog", demo_dialog::render),
    ("rust/key-bar", demo_key_bar::render),
    ("rust/panel", demo_panel::render),
    ("rust/radio-card", demo_radio_card::render),
    ("rust/select-list", demo_select_list::render),
    ("rust/text-input", demo_text_input::render),
    ("rust/toast", demo_toast::render),
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
