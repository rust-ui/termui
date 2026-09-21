use ratatui::style::{Color, Modifier};
use termui_widgets::button::{Button, ButtonSize, ButtonVariant};

pub(super) fn default() -> Vec<String> {
    vec![button(
        "Button",
        ButtonVariant::Default,
        ButtonSize::Default,
    )]
}

pub(super) fn variants() -> Vec<String> {
    vec![
        button_row(&[
            ("Default", ButtonVariant::Default, ButtonSize::Default),
            ("Secondary", ButtonVariant::Secondary, ButtonSize::Default),
            ("Delete", ButtonVariant::Destructive, ButtonSize::Default),
        ]),
        button_row(&[
            ("Outline", ButtonVariant::Outline, ButtonSize::Default),
            ("Ghost", ButtonVariant::Ghost, ButtonSize::Default),
            ("Link", ButtonVariant::Link, ButtonSize::Default),
        ]),
    ]
}

pub(super) fn sizes() -> Vec<String> {
    vec![button_row(&[
        ("Small", ButtonVariant::Default, ButtonSize::Sm),
        ("Default", ButtonVariant::Default, ButtonSize::Default),
        ("Large", ButtonVariant::Default, ButtonSize::Lg),
    ])]
}

fn button_row(buttons: &[(&str, ButtonVariant, ButtonSize)]) -> String {
    buttons
        .iter()
        .map(|(label, variant, size)| button(label, *variant, *size))
        .collect::<Vec<_>>()
        .join("   ")
}

fn button(label: &str, variant: ButtonVariant, size: ButtonSize) -> String {
    let line = Button::new(label).variant(variant).size(size).line();
    line.spans
        .iter()
        .map(|span| {
            let style = span.style;
            let mut codes = Vec::new();
            push_color_code(&mut codes, style.fg, false);
            push_color_code(&mut codes, style.bg, true);
            if style.add_modifier.contains(Modifier::BOLD) {
                codes.push("1".to_string());
            }
            if style.add_modifier.contains(Modifier::DIM) {
                codes.push("2".to_string());
            }
            if style.add_modifier.contains(Modifier::UNDERLINED) {
                codes.push("4".to_string());
            }

            if codes.is_empty() {
                span.content.to_string()
            } else {
                format!("\x1b[{}m{}\x1b[0m", codes.join(";"), span.content)
            }
        })
        .collect()
}

fn push_color_code(codes: &mut Vec<String>, color: Option<Color>, background: bool) {
    let Some(Color::Rgb(red, green, blue)) = color else {
        return;
    };
    let channel = if background { 48 } else { 38 };
    codes.push(format!("{channel};2;{red};{green};{blue}"));
}
