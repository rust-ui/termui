const WIDTH: usize = 42;

fn fit(text: &str, width: usize) -> String {
    let text = text.chars().take(width).collect::<String>();
    format!(" {text:<width$}")
}

fn row(text: &str) -> String {
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

    if name.contains("chart") || name.contains("sparkline") || name.contains("gauge") {
        lines.extend([
            row("  ▂   ▄   ▃   ▆   ▅   ▇   ▄   █"),
            row("  12  18  15  28  24  36  21  42"),
        ]);
    } else if name.contains("table") || name.contains("grid") || name.contains("list") {
        lines.extend([
            row("  NAME          STATUS       VALUE"),
            row("  alpha         ready          12"),
            row("  beta          running        24"),
        ]);
    } else if name.contains("input") || name.contains("form") || name.contains("picker") {
        lines.extend([
            row("  > Enter a value: rust|"),
            row("    Press Enter to continue"),
        ]);
    } else {
        lines.extend([
            row("  Ready                           ✓"),
            row("  Interactive Rust example"),
        ]);
    }

    lines.push(format!("╰{}╯", "─".repeat(WIDTH - 2)));
    lines
}
