#[path = "demo_support/button_helpers.rs"]
mod button_helpers;
#[path = "demos/demo_button.rs"]
mod demo_button;
#[path = "demos/demo_button_sizes.rs"]
mod demo_button_sizes;
#[path = "demos/demo_button_variants.rs"]
mod demo_button_variants;
#[path = "demos/demo_key_bar.rs"]
mod demo_key_bar;
#[path = "demos/demo_panel.rs"]
mod demo_panel;
#[path = "demos/demo_select_list.rs"]
mod demo_select_list;

type DemoRenderer = fn() -> Vec<String>;

const DEMOS: &[(&str, DemoRenderer)] = &[
    ("rust/button", demo_button::render),
    ("rust/button-sizes", demo_button_sizes::render),
    ("rust/button-variants", demo_button_variants::render),
    ("rust/key-bar", demo_key_bar::render),
    ("rust/panel", demo_panel::render),
    ("rust/select-list", demo_select_list::render),
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
