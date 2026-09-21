const WIDTH: usize = 42;

fn fit(text: &str, width: usize) -> String {
    let visible_width = text
        .split("\x1b[")
        .enumerate()
        .map(|(index, part)| {
            if index == 0 {
                part.chars().count()
            } else {
                part.split_once('m').map_or(0, |(_, rest)| rest.chars().count())
            }
        })
        .sum::<usize>();
    format!(" {text}{}", " ".repeat(width.saturating_sub(visible_width)))
}

fn row(text: &str) -> String {
    format!("│{}│", fit(text, WIDTH - 3))
}

fn button(label: &str, foreground: &str, background: &str, modifiers: &str) -> String {
    let background = if background.is_empty() {
        String::new()
    } else {
        format!(";48;2;{background}")
    };
    format!("\x1b[{modifiers};38;2;{foreground}{background}m{label}\x1b[0m")
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

    if name.ends_with("/button") {
        lines.extend([
            row(&format!(
                "  {}  {}",
                button(" Save ", "250;250;250", "37;99;235", "1"),
                button(" Cancel ", "228;228;231", "63;63;70", "0")
            )),
            row(&format!(
                "  {}  {}  {}",
                button(" Delete ", "250;250;250", "220;38;38", "1"),
                button("[Outline]", "228;228;231", "", "4"),
                button("Ghost", "228;228;231", "", "0")
            )),
            row(&format!(
                "  {}  {}",
                button("Link", "96;165;250", "", "4"),
                button(" Disabled ", "228;228;231", "37;99;235", "2")
            )),
        ]);
    } else if name.contains("chart") || name.contains("sparkline") || name.contains("gauge") {
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
