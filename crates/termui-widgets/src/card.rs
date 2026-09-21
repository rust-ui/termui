use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use ratatui::Frame;

use crate::panel::Panel;

/// Rounded card shell. Render child components into the returned regions.
#[derive(Debug, Clone)]
#[must_use]
pub struct Card {
    header_rows: u16,
    footer_rows: u16,
    border_style: Style,
}

/// Regions exposed by [`Card`] for composing card children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CardAreas {
    pub outer: Rect,
    pub header: Rect,
    pub content: Rect,
    pub footer: Rect,
}

impl Card {
    pub fn new() -> Self {
        Self {
            header_rows: 3,
            footer_rows: 2,
            border_style: Style::default().fg(Color::Rgb(63, 63, 70)),
        }
    }

    pub fn header_rows(mut self, rows: u16) -> Self {
        self.header_rows = rows;
        self
    }

    pub fn footer_rows(mut self, rows: u16) -> Self {
        self.footer_rows = rows;
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) -> CardAreas {
        let block = Panel::new().border_style(self.border_style).block();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let footer_rows = self.footer_rows.min(inner.height);
        let header_rows = self
            .header_rows
            .min(inner.height.saturating_sub(footer_rows));
        let [header, content, footer] = Layout::vertical([
            Constraint::Length(header_rows),
            Constraint::Min(0),
            Constraint::Length(footer_rows),
        ])
        .areas(inner);

        CardAreas {
            outer: area,
            header,
            content,
            footer: if footer_rows == 0 {
                Rect::default()
            } else {
                footer
            },
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

/// Splits a card header into title and description regions.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct CardHeader {
    description_rows: u16,
    horizontal_padding: u16,
}

/// Regions exposed by [`CardHeader`] for composing its parts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CardHeaderAreas {
    pub title: Rect,
    pub description: Rect,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            description_rows: 1,
            horizontal_padding: 1,
        }
    }

    pub fn description_rows(mut self, rows: u16) -> Self {
        self.description_rows = rows;
        self
    }

    pub fn horizontal_padding(mut self, padding: u16) -> Self {
        self.horizontal_padding = padding;
        self
    }

    pub fn layout(self, area: Rect) -> CardHeaderAreas {
        let padding = self.horizontal_padding.min(area.width / 2);
        let inner = Rect::new(
            area.x.saturating_add(padding),
            area.y,
            area.width.saturating_sub(padding.saturating_mul(2)),
            area.height,
        );
        let description_rows = self.description_rows.min(inner.height);
        let title_rows = inner.height.saturating_sub(description_rows);
        let [title, description] = Layout::vertical([
            Constraint::Length(title_rows),
            Constraint::Length(description_rows),
        ])
        .areas(inner);

        CardHeaderAreas { title, description }
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Prominent card heading.
#[must_use]
pub struct CardTitle<'a> {
    title: Line<'a>,
}

impl<'a> CardTitle<'a> {
    pub fn new<T: Into<Line<'a>>>(title: T) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.title).style(
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
            area,
        );
    }
}

/// Muted supporting copy for a card header.
#[must_use]
pub struct CardDescription<'a> {
    text: Text<'a>,
}

impl<'a> CardDescription<'a> {
    pub fn new<T: Into<Text<'a>>>(text: T) -> Self {
        Self { text: text.into() }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.text)
                .style(Style::default().fg(Color::Gray))
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

/// Main card body wrapper with horizontal padding.
#[must_use]
pub struct CardContent<W> {
    child: W,
    horizontal_padding: u16,
}

impl<W> CardContent<W> {
    pub fn new(child: W) -> Self {
        Self {
            child,
            horizontal_padding: 1,
        }
    }

    pub fn horizontal_padding(mut self, padding: u16) -> Self {
        self.horizontal_padding = padding;
        self
    }
}

impl<W: Widget> CardContent<W> {
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let padding = self.horizontal_padding.min(area.width / 2);
        let content = Rect::new(
            area.x.saturating_add(padding),
            area.y,
            area.width.saturating_sub(padding.saturating_mul(2)),
            area.height,
        );
        frame.render_widget(self.child, content);
    }
}

/// Card footer separator; returns padded area for footer children.
#[must_use]
pub struct CardFooter {
    horizontal_padding: u16,
    separator_style: Style,
}

impl CardFooter {
    pub fn new() -> Self {
        Self {
            horizontal_padding: 1,
            separator_style: Style::default().fg(Color::Rgb(63, 63, 70)),
        }
    }

    pub fn horizontal_padding(mut self, padding: u16) -> Self {
        self.horizontal_padding = padding;
        self
    }

    pub fn separator_style(mut self, style: Style) -> Self {
        self.separator_style = style;
        self
    }
}

impl CardFooter {
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) -> Rect {
        if area.height == 0 {
            return area;
        }
        frame.render_widget(
            Paragraph::new(Line::styled(
                "─".repeat(usize::from(area.width)),
                self.separator_style,
            )),
            Rect::new(area.x, area.y, area.width, 1),
        );

        let padding = self.horizontal_padding.min(area.width / 2);
        Rect::new(
            area.x.saturating_add(padding),
            area.y.saturating_add(1),
            area.width.saturating_sub(padding.saturating_mul(2)),
            area.height.saturating_sub(1),
        )
    }
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
pub fn __render_card_markup(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    description: &str,
    content: &str,
    footer: &str,
) {
    let areas = Card::new()
        .header_rows(2)
        .footer_rows(2)
        .render(frame, area);
    let header = CardHeader::new().description_rows(1).layout(areas.header);

    CardTitle::new(title).render(frame, header.title);
    CardDescription::new(description).render(frame, header.description);
    CardContent::new(Paragraph::new(content)).render(frame, areas.content);
    let footer_area = CardFooter::new().render(frame, areas.footer);
    frame.render_widget(Paragraph::new(footer), footer_area);
}

/// Render a Card from an RSX-style component tree.
///
/// The first version accepts literal text and the standard Card slots.
#[macro_export]
macro_rules! termui {
    (
        frame: $frame:expr,
        area: $area:expr,
        Card {
            CardHeader {
                CardTitle { $title:literal }
                CardDescription { $description:literal }
            }
            CardContent {
                p { $content:literal }
            }
            CardFooter {
                p { $footer:literal }
            }
        }
    ) => {{
        $crate::card::__render_card_markup($frame, $area, $title, $description, $content, $footer)
    }};
}
