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

/// Splits a card header into title, description, and optional action regions.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct CardHeader {
    description_rows: u16,
    action_width: u16,
    horizontal_padding: u16,
}

/// Regions exposed by [`CardHeader`] for composing its parts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CardHeaderAreas {
    pub title: Rect,
    pub description: Rect,
    pub action: Rect,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            description_rows: 1,
            action_width: 0,
            horizontal_padding: 1,
        }
    }

    pub fn description_rows(mut self, rows: u16) -> Self {
        self.description_rows = rows;
        self
    }

    pub fn action_width(mut self, width: u16) -> Self {
        self.action_width = width;
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
        let action_width = self.action_width.min(inner.width);
        let [main, action] =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(action_width)]).areas(inner);
        let description_rows = self.description_rows.min(main.height);
        let title_rows = main.height.saturating_sub(description_rows);
        let [title, description] = Layout::vertical([
            Constraint::Length(title_rows),
            Constraint::Length(description_rows),
        ])
        .areas(main);

        CardHeaderAreas {
            title,
            description,
            action: if action_width == 0 {
                Rect::default()
            } else {
                action
            },
        }
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
                    .fg(Color::White)
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

/// Action child wrapper for a card header.
#[must_use]
pub struct CardAction<W> {
    child: W,
}

impl<W> CardAction<W> {
    pub fn new(child: W) -> Self {
        Self { child }
    }
}

impl<W: Widget> CardAction<W> {
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(self.child, area);
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

/// Card footer with separator and a padded child widget.
#[must_use]
pub struct CardFooter<W> {
    child: W,
    horizontal_padding: u16,
    separator_style: Style,
}

impl<W> CardFooter<W> {
    pub fn new(child: W) -> Self {
        Self {
            child,
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

impl<W: Widget> CardFooter<W> {
    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        if area.height == 0 {
            return;
        }
        frame.render_widget(
            Paragraph::new(Line::styled(
                "─".repeat(usize::from(area.width)),
                self.separator_style,
            )),
            Rect::new(area.x, area.y, area.width, 1),
        );

        let padding = self.horizontal_padding.min(area.width / 2);
        let child = Rect::new(
            area.x.saturating_add(padding),
            area.y.saturating_add(1),
            area.width.saturating_sub(padding.saturating_mul(2)),
            area.height.saturating_sub(1),
        );
        frame.render_widget(self.child, child);
    }
}
