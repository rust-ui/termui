use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

/// A small Markdown presentation for headings, bold text, and bullet lists.
#[must_use]
pub struct MarkdownViewer<'a> {
    markdown: &'a str,
}

impl<'a> MarkdownViewer<'a> {
    pub fn new(markdown: &'a str) -> Self {
        Self { markdown }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let lines = self
            .markdown
            .lines()
            .map(|line| {
                if let Some(heading) = line.strip_prefix("# ") {
                    Line::from(parse_inline(
                        heading,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else if let Some(item) =
                    line.strip_prefix("- ").or_else(|| line.strip_prefix("* "))
                {
                    let mut spans = vec![Span::styled("• ", Style::default().fg(Color::Cyan))];
                    spans.extend(parse_inline(item, Style::default()));
                    Line::from(spans)
                } else {
                    Line::from(parse_inline(line, Style::default()))
                }
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
    }
}

fn parse_inline(text: &str, base_style: Style) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut remaining = text;
    while let Some(start) = remaining.find("**") {
        let after_marker = &remaining[start + 2..];
        let Some(end) = after_marker.find("**") else {
            break;
        };
        if start > 0 {
            spans.push(Span::styled(remaining[..start].to_owned(), base_style));
        }
        spans.push(Span::styled(
            after_marker[..end].to_owned(),
            base_style.add_modifier(Modifier::BOLD),
        ));
        remaining = &after_marker[end + 2..];
    }
    if !remaining.is_empty() {
        spans.push(Span::styled(remaining.to_owned(), base_style));
    }
    spans
}
