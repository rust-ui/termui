use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Clear};

/// Child regions returned by a rendered dialog shell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DialogAreas {
    pub outer: Rect,
    pub body: Rect,
    pub footer: Rect,
}

/// Centered modal shell. Render body and footer widgets into the returned areas.
#[must_use]
pub struct Dialog<'a> {
    title: Line<'a>,
    width: u16,
    height: u16,
    footer_rows: u16,
    border_style: Style,
}

impl<'a> Dialog<'a> {
    pub fn new<T: Into<Line<'a>>>(title: T) -> Self {
        Self {
            title: title.into(),
            width: u16::MAX,
            height: u16::MAX,
            footer_rows: 0,
            border_style: Style::default(),
        }
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
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

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) -> DialogAreas {
        let width = self.width.min(area.width);
        let height = self.height.min(area.height);
        let outer = Rect::new(
            area.x + area.width.saturating_sub(width) / 2,
            area.y + area.height.saturating_sub(height) / 2,
            width,
            height,
        );
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.border_style)
            .title(self.title);
        let inner = block.inner(outer);

        frame.render_widget(Clear, outer);
        frame.render_widget(block, outer);

        let footer_rows = self.footer_rows.min(inner.height);
        let [body, footer] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(footer_rows)]).areas(inner);
        DialogAreas {
            outer,
            body,
            footer: if footer_rows == 0 {
                Rect::default()
            } else {
                footer
            },
        }
    }
}
