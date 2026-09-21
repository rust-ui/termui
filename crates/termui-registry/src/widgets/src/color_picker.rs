use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

const COLORS: [(&str, Color); 8] = [
    ("Slate", Color::Rgb(100, 116, 139)),
    ("Gray", Color::Rgb(107, 114, 128)),
    ("Red", Color::Rgb(239, 68, 68)),
    ("Orange", Color::Rgb(249, 115, 22)),
    ("Green", Color::Rgb(34, 197, 94)),
    ("Teal", Color::Rgb(20, 184, 166)),
    ("Blue", Color::Rgb(59, 130, 246)),
    ("Violet", Color::Rgb(139, 92, 246)),
];

/// Compact palette preview. The parent owns selection and color application.
#[must_use]
pub struct ColorPicker {
    selected: usize,
}

impl ColorPicker {
    pub fn new(selected: usize) -> Self {
        Self {
            selected: selected.min(COLORS.len().saturating_sub(1)),
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Accent color ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::with_capacity(3);
        for row in COLORS.chunks(4) {
            let mut spans = Vec::with_capacity(row.len() * 2);
            for (index, (name, color)) in row.iter().enumerate() {
                let absolute_index = if lines.is_empty() { index } else { index + 4 };
                let selected = absolute_index == self.selected;
                spans.push(Span::styled(
                    if selected { "▣ " } else { "● " },
                    Style::default().fg(*color).add_modifier(if selected {
                        Modifier::BOLD | Modifier::UNDERLINED
                    } else {
                        Modifier::empty()
                    }),
                ));
                spans.push(Span::styled(
                    format!("{name:<7}"),
                    Style::default().fg(if selected { *color } else { Color::Gray }),
                ));
            }
            lines.push(Line::from(spans));
        }
        let (name, color) = COLORS[self.selected];
        let (red, green, blue) = rgb(color);
        lines.push(Line::from(vec![
            Span::styled("Selected ", Style::default().fg(Color::Gray)),
            Span::styled("  ", Style::default().bg(color)),
            Span::raw(" "),
            Span::raw(format!("{name}  #{red:02X}{green:02X}{blue:02X}")),
        ]));
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

fn rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(red, green, blue) => (red, green, blue),
        _ => (0, 0, 0),
    }
}
