mod demos;

pub(crate) const WIDTH: usize = 42;

fn fit(text: &str, width: usize) -> String {
    let visible_width = text
        .split("\x1b[")
        .enumerate()
        .map(|(index, part)| {
            if index == 0 {
                part.chars().count()
            } else {
                part.split_once('m')
                    .map_or(0, |(_, rest)| rest.chars().count())
            }
        })
        .sum::<usize>();
    format!(" {text}{}", " ".repeat(width.saturating_sub(visible_width)))
}

pub(crate) fn row(text: &str) -> String {
    format!("│{}│", fit(text, WIDTH - 3))
}

fn humanize(name: &str) -> String {
    name.rsplit('/')
        .next()
        .unwrap_or(name)
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render one named example into terminal-cell text lines.
pub fn render_demo(name: &str) -> Vec<String> {
    if let Some(frame) = demos::render(name) {
        return frame;
    }

    render_generic_demo(name)
}

fn render_generic_demo(name: &str) -> Vec<String> {
    let title = humanize(name);
    let base = name.split('/').next().unwrap_or("Rust");
    let label = match base {
        "opentui" => "OpenTUI",
        "rust" => "Rust",
        _ => "Ink",
    };
    let title_room = WIDTH.saturating_sub(title.chars().count() + 4);
    let mut lines = vec![
        format!("╭─ {}{}╮", title, "─".repeat(title_room)),
        row("Rust-rendered terminal component"),
        row(&format!("Example: {title}")),
        row(&format!("Docs variant: {label}")),
    ];

    match classify_generic_demo(name) {
        GenericDemoKind::Chart => lines.extend([
            row("  ▂   ▄   ▃   ▆   ▅   ▇   ▄   █"),
            row("  12  18  15  28  24  36  21  42"),
        ]),
        GenericDemoKind::Table => lines.extend([
            row("  NAME          STATUS       VALUE"),
            row("  alpha         ready          12"),
            row("  beta          running        24"),
        ]),
        GenericDemoKind::Input => lines.extend([
            row("  > Enter a value: rust|"),
            row("    Press Enter to continue"),
        ]),
        GenericDemoKind::Default => lines.extend([
            row("  Ready                           ✓"),
            row("  Interactive Rust example"),
        ]),
    }

    lines.push(format!("╰{}╯", "─".repeat(WIDTH - 2)));
    lines
}

#[derive(Clone, Copy)]
enum GenericDemoKind {
    Chart,
    Table,
    Input,
    Default,
}

fn classify_generic_demo(name: &str) -> GenericDemoKind {
    const CLASSIFIERS: &[(&[&str], GenericDemoKind)] = &[
        (&["chart", "sparkline", "gauge"], GenericDemoKind::Chart),
        (&["table", "grid", "list"], GenericDemoKind::Table),
        (&["input", "form", "picker"], GenericDemoKind::Input),
    ];

    CLASSIFIERS
        .iter()
        .find(|(patterns, _)| patterns.iter().any(|pattern| name.contains(pattern)))
        .map_or(GenericDemoKind::Default, |(_, kind)| *kind)
}
