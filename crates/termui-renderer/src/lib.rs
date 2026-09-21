use ratatui::{
    backend::TestBackend,
    buffer::{Buffer, Cell},
    style::{Color, Modifier},
    Frame, Terminal,
};

pub const PREVIEW_WIDTH: u16 = 42;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CellStyle {
    foreground: Color,
    background: Color,
    modifiers: Modifier,
}

impl From<&Cell> for CellStyle {
    fn from(cell: &Cell) -> Self {
        Self {
            foreground: cell.fg,
            background: cell.bg,
            modifiers: cell.modifier,
        }
    }
}

/// Draw a Ratatui frame into a fixed terminal buffer and serialize its cells as ANSI text.
pub fn render_frame(height: u16, draw: impl FnOnce(&mut Frame<'_>)) -> Vec<String> {
    let backend = TestBackend::new(PREVIEW_WIDTH, height);
    let mut terminal = Terminal::new(backend).expect("create Ratatui preview backend");
    terminal.draw(draw).expect("draw Ratatui preview frame");
    buffer_lines(terminal.backend().buffer())
}

fn buffer_lines(buffer: &Buffer) -> Vec<String> {
    let lines = (0..buffer.area.height)
        .map(|y| {
            let row_end = (0..buffer.area.width)
                .rev()
                .find(|x| {
                    let cell = &buffer[(*x, y)];
                    cell.symbol() != " "
                        || cell.fg != Color::Reset
                        || cell.bg != Color::Reset
                        || !cell.modifier.is_empty()
                })
                .map_or(0, |x| x + 1);

            let mut line = String::new();
            let mut active_style = CellStyle::default();
            for x in 0..row_end {
                let cell = &buffer[(x, y)];
                let next_style = CellStyle::from(cell);
                if next_style != active_style {
                    line.push_str("\x1b[0m");
                    let codes = style_codes(next_style);
                    if !codes.is_empty() {
                        line.push_str("\x1b[");
                        line.push_str(&codes.join(";"));
                        line.push('m');
                    }
                    active_style = next_style;
                }
                line.push_str(cell.symbol());
            }
            if active_style != CellStyle::default() {
                line.push_str("\x1b[0m");
            }
            line
        })
        .collect::<Vec<_>>();
    let first = lines
        .iter()
        .position(|line| !line.is_empty())
        .unwrap_or(lines.len());
    let last = lines
        .iter()
        .rposition(|line| !line.is_empty())
        .map_or(first, |index| index + 1);
    lines[first..last].to_vec()
}

fn style_codes(style: CellStyle) -> Vec<String> {
    let mut codes = Vec::new();
    push_color_code(&mut codes, style.foreground, false);
    push_color_code(&mut codes, style.background, true);

    for (modifier, code) in [
        (Modifier::BOLD, 1),
        (Modifier::DIM, 2),
        (Modifier::ITALIC, 3),
        (Modifier::UNDERLINED, 4),
        (Modifier::SLOW_BLINK, 5),
        (Modifier::RAPID_BLINK, 6),
        (Modifier::REVERSED, 7),
        (Modifier::HIDDEN, 8),
        (Modifier::CROSSED_OUT, 9),
    ] {
        if style.modifiers.contains(modifier) {
            codes.push(code.to_string());
        }
    }
    codes
}

fn push_color_code(codes: &mut Vec<String>, color: Color, background: bool) {
    let channel = if background { 48 } else { 38 };
    match color {
        Color::Reset => {}
        Color::Black
        | Color::Red
        | Color::Green
        | Color::Yellow
        | Color::Blue
        | Color::Magenta
        | Color::Cyan
        | Color::Gray => {
            let index = match color {
                Color::Black => 0,
                Color::Red => 1,
                Color::Green => 2,
                Color::Yellow => 3,
                Color::Blue => 4,
                Color::Magenta => 5,
                Color::Cyan => 6,
                Color::Gray => 7,
                _ => unreachable!(),
            };
            codes.push(((if background { 40 } else { 30 }) + index).to_string());
        }
        Color::DarkGray
        | Color::LightRed
        | Color::LightGreen
        | Color::LightYellow
        | Color::LightBlue
        | Color::LightMagenta
        | Color::LightCyan
        | Color::White => {
            let index = match color {
                Color::DarkGray => 0,
                Color::LightRed => 1,
                Color::LightGreen => 2,
                Color::LightYellow => 3,
                Color::LightBlue => 4,
                Color::LightMagenta => 5,
                Color::LightCyan => 6,
                Color::White => 7,
                _ => unreachable!(),
            };
            codes.push(((if background { 100 } else { 90 }) + index).to_string());
        }
        Color::Indexed(index) => codes.extend([channel.to_string(), "5".into(), index.to_string()]),
        Color::Rgb(red, green, blue) => codes.extend([
            channel.to_string(),
            "2".into(),
            red.to_string(),
            green.to_string(),
            blue.to_string(),
        ]),
    }
}

fn row(text: &str) -> String {
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
    format!(
        "│ {text}{}│",
        " ".repeat((PREVIEW_WIDTH as usize - 3).saturating_sub(visible_width))
    )
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

pub fn render_example_preview(name: &str) -> Vec<String> {
    let title = humanize(name);
    let title_room = (PREVIEW_WIDTH as usize).saturating_sub(title.chars().count() + 4);
    let mut lines = vec![
        format!("╭─ {}{}╮", title, "─".repeat(title_room)),
        row("Rust-rendered terminal component"),
        row(&format!("Example: {title}")),
        row("Ratatui widget preview"),
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

    lines.push(format!("╰{}╯", "─".repeat(PREVIEW_WIDTH as usize - 2)));
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
