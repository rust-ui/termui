mod button;

pub(super) fn render(name: &str) -> Option<Vec<String>> {
    match name {
        "rust/button" => Some(button::default()),
        "rust/button-variants" => Some(button::variants()),
        "rust/button-sizes" => Some(button::sizes()),
        _ => None,
    }
}
