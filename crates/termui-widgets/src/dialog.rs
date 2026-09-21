use std::time::Duration;

use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::Frame;

use crate::button::{Button, ButtonVariant};
use crate::tui_overlay::anchor::Anchor;
use crate::tui_overlay::backdrop::Backdrop;
use crate::tui_overlay::overlay::Overlay;
use crate::tui_overlay::slide::Slide;
use crate::tui_overlay::state::OverlayState;

const DEFAULT_ANIMATION_TIME: Duration = Duration::from_millis(180);

/// Owns open state and animation for a composed dialog.
#[derive(Debug, Clone)]
pub struct Dialog {
    state: OverlayState,
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            state: OverlayState::new().with_duration(DEFAULT_ANIMATION_TIME),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.state = self.state.with_duration(duration);
        self
    }

    pub fn open(&mut self) {
        self.state.open();
    }

    pub fn close(&mut self) {
        self.state.close();
    }

    pub fn toggle(&mut self) {
        self.state.toggle();
    }

    pub fn tick(&mut self, elapsed: Duration) {
        self.state.tick(elapsed);
    }

    pub fn is_open(&self) -> bool {
        self.state.is_open()
    }

    pub fn is_closed(&self) -> bool {
        self.state.is_closed()
    }

    pub fn is_animating(&self) -> bool {
        self.state.is_animating()
    }

    /// Close when a click lands outside the dialog. Returns whether it closed.
    pub fn close_on_outside_click(&mut self, position: Position) -> bool {
        let Some(rect) = self.state.overlay_rect() else {
            return false;
        };
        if rect.contains(position) {
            return false;
        }
        self.close();
        true
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

/// Button that opens a parent-owned [`Dialog`] when activated.
#[must_use]
pub struct DialogTrigger<'a> {
    button: Button<'a>,
}

impl<'a> DialogTrigger<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            button: Button::new(label).variant(ButtonVariant::Default),
        }
    }

    pub fn activate(&self, dialog: &mut Dialog) {
        dialog.open();
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.button = self.button.focused(focused);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        self.button.render(frame, area);
    }
}

/// Animated modal surface. Render child components into the returned regions.
#[must_use]
pub struct DialogContent {
    width: Constraint,
    height: Constraint,
    header_rows: u16,
    footer_rows: u16,
}

/// Regions exposed by [`DialogContent`] for composing dialog children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DialogAreas {
    pub outer: Rect,
    pub header: Rect,
    pub body: Rect,
    pub footer: Rect,
}

impl DialogContent {
    pub fn new() -> Self {
        Self {
            width: Constraint::Length(52),
            height: Constraint::Length(12),
            header_rows: 3,
            footer_rows: 3,
        }
    }

    pub fn width(mut self, width: Constraint) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Constraint) -> Self {
        self.height = height;
        self
    }

    pub fn header_rows(mut self, rows: u16) -> Self {
        self.header_rows = rows;
        self
    }

    pub fn footer_rows(mut self, rows: u16) -> Self {
        self.footer_rows = rows;
        self
    }

    pub fn render(
        self,
        frame: &mut Frame<'_>,
        area: Rect,
        dialog: &mut Dialog,
    ) -> Option<DialogAreas> {
        let border = Style::default().fg(Color::Rgb(63, 63, 70));
        let overlay = Overlay::new()
            .anchor(Anchor::Center)
            .slide(Slide::Bottom)
            .backdrop(Backdrop::new(Color::Black).fg(Color::DarkGray))
            .width(self.width)
            .height(self.height)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(border),
            )
            .bg(Color::Rgb(9, 9, 11));
        frame.render_stateful_widget(overlay, area, &mut dialog.state);

        let outer = dialog.state.overlay_rect()?;
        let inner = dialog.state.inner_area()?;
        let footer_rows = self.footer_rows.min(inner.height);
        let header_rows = self
            .header_rows
            .min(inner.height.saturating_sub(footer_rows));
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(header_rows),
            Constraint::Min(0),
            Constraint::Length(footer_rows),
        ])
        .areas(inner);

        Some(DialogAreas {
            outer,
            header,
            body,
            footer: if footer_rows == 0 {
                Rect::default()
            } else {
                footer
            },
        })
    }
}

impl Default for DialogContent {
    fn default() -> Self {
        Self::new()
    }
}

/// Header that composes a [`DialogTitle`] and optional [`DialogDescription`].
#[must_use]
pub struct DialogHeader<'a> {
    title: DialogTitle<'a>,
    description: Option<DialogDescription<'a>>,
}

impl<'a> DialogHeader<'a> {
    pub fn new(title: DialogTitle<'a>) -> Self {
        Self {
            title,
            description: None,
        }
    }

    pub fn description(mut self, description: DialogDescription<'a>) -> Self {
        self.description = Some(description);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let description_rows = if self.description.is_some() {
            area.height.saturating_sub(1)
        } else {
            0
        };
        let [title, description] = Layout::vertical([
            Constraint::Length(area.height.saturating_sub(description_rows)),
            Constraint::Length(description_rows),
        ])
        .areas(area);
        self.title.render(frame, title);
        if let Some(description_widget) = self.description {
            description_widget.render(frame, description);
        }
    }
}

/// Prominent dialog heading.
#[must_use]
pub struct DialogTitle<'a> {
    title: Line<'a>,
}

impl<'a> DialogTitle<'a> {
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

/// Muted supporting copy for a dialog header.
#[must_use]
pub struct DialogDescription<'a> {
    text: Text<'a>,
}

impl<'a> DialogDescription<'a> {
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

/// Footer separator; returns area for action buttons.
#[must_use]
pub struct DialogFooter;

impl DialogFooter {
    pub fn new() -> Self {
        Self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) -> Rect {
        if area.height == 0 {
            return area;
        }
        frame.render_widget(
            Paragraph::new(Line::styled(
                "─".repeat(usize::from(area.width)),
                Style::default().fg(Color::Rgb(63, 63, 70)),
            )),
            Rect::new(area.x, area.y, area.width, 1),
        );
        Rect::new(
            area.x,
            area.y.saturating_add(1),
            area.width,
            area.height.saturating_sub(1),
        )
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

/// Button that closes a parent-owned [`Dialog`] when activated.
#[must_use]
pub struct DialogClose<'a> {
    button: Button<'a>,
}

impl<'a> DialogClose<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            button: Button::new(label).variant(ButtonVariant::Secondary),
        }
    }

    pub fn activate(&self, dialog: &mut Dialog) {
        dialog.close();
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.button = self.button.focused(focused);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        self.button.render(frame, area);
    }
}
