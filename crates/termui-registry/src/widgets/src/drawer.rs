use std::time::Duration;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

use crate::button::{Button, ButtonVariant};
use crate::tui_overlay::anchor::Anchor;
use crate::tui_overlay::backdrop::Backdrop;
use crate::tui_overlay::overlay::Overlay;
use crate::tui_overlay::slide::Slide;
use crate::tui_overlay::state::OverlayState;

const DEFAULT_ANIMATION_TIME: Duration = Duration::from_millis(180);
const DRAWER_BACKGROUND: Color = Color::Rgb(24, 24, 27);

/// Owns visibility and animation for a composable side drawer.
#[derive(Debug, Clone)]
pub struct Drawer {
    state: OverlayState,
}

impl Drawer {
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

    /// Close when a click lands outside the rendered drawer. Returns whether dismissal was requested.
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

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

/// Left or right edge used by [`DrawerContent`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Left,
    #[default]
    Right,
}

impl DrawerSide {
    fn anchor(self) -> Anchor {
        match self {
            Self::Left => Anchor::Left,
            Self::Right => Anchor::Right,
        }
    }

    fn slide(self) -> Slide {
        match self {
            Self::Left => Slide::Left,
            Self::Right => Slide::Right,
        }
    }
}

/// Button that opens a parent-owned [`Drawer`] when activated.
#[must_use]
pub struct DrawerTrigger<'a> {
    button: Button<'a>,
}

impl<'a> DrawerTrigger<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            button: Button::new(label).variant(ButtonVariant::Default),
        }
    }

    pub fn activate(&self, drawer: &mut Drawer) {
        drawer.open();
    }

    pub fn style(mut self, style: Style) -> Self {
        self.button = self.button.style(style);
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.button = self.button.focused(focused);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        self.button.render(frame, area);
    }
}

/// Animated edge panel. Child components render into the returned areas.
#[must_use]
pub struct DrawerContent {
    width: Constraint,
    side: DrawerSide,
}

/// Regions exposed by [`DrawerContent`] for composing drawer children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrawerAreas {
    pub outer: Rect,
    pub title: Rect,
    pub description: Rect,
    pub body: Rect,
    pub close: Rect,
}

impl DrawerContent {
    pub fn new() -> Self {
        Self {
            width: Constraint::Percentage(45),
            side: DrawerSide::Right,
        }
    }

    pub fn width(mut self, width: Constraint) -> Self {
        self.width = width;
        self
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn render(
        self,
        frame: &mut Frame<'_>,
        area: Rect,
        drawer: &mut Drawer,
    ) -> Option<DrawerAreas> {
        let border = Style::default().fg(Color::Rgb(63, 63, 70));
        let width = self.width;
        let overlay = Overlay::new()
            .anchor(self.side.anchor())
            .slide(self.side.slide())
            .width(width)
            .height(Constraint::Percentage(100))
            .backdrop(Backdrop::new(Color::Black).fg(Color::DarkGray))
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(border),
            )
            .bg(DRAWER_BACKGROUND);
        frame.render_stateful_widget(overlay, area, &mut drawer.state);

        let outer = drawer.state.overlay_rect()?;
        let inner = drawer.state.inner_area()?;
        let header_rows = 1.min(inner.height);
        let description_rows = 2.min(inner.height.saturating_sub(header_rows));
        let [header, description, body] = Layout::vertical([
            Constraint::Length(header_rows),
            Constraint::Length(description_rows),
            Constraint::Min(0),
        ])
        .areas(inner);
        let close_width = 5.min(header.width);
        let close = Rect::new(
            header.right().saturating_sub(close_width),
            header.y,
            close_width,
            header.height,
        );
        let title = Rect::new(
            header.x,
            header.y,
            header.width.saturating_sub(close_width),
            header.height,
        );

        Some(DrawerAreas {
            outer,
            title,
            description,
            body,
            close,
        })
    }
}

impl Default for DrawerContent {
    fn default() -> Self {
        Self::new()
    }
}

/// Prominent drawer heading.
#[must_use]
pub struct DrawerTitle<'a> {
    text: &'a str,
}

impl<'a> DrawerTitle<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.text).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            area,
        );
    }
}

/// Muted supporting copy for a drawer heading.
#[must_use]
pub struct DrawerDescription<'a> {
    text: Text<'a>,
}

impl<'a> DrawerDescription<'a> {
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

/// Button that closes a parent-owned [`Drawer`] when activated.
#[must_use]
pub struct DrawerClose<'a> {
    button: Button<'a>,
}

impl<'a> DrawerClose<'a> {
    pub fn new() -> Self {
        Self {
            button: Button::new("x").variant(ButtonVariant::Ghost),
        }
    }

    pub fn activate(&self, drawer: &mut Drawer) {
        drawer.close();
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.button = self.button.focused(focused);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        self.button.render(frame, area);
    }
}

impl Default for DrawerClose<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn outside_click_closes_drawer_but_inside_click_does_not() {
        let mut drawer = Drawer::new().with_duration(Duration::ZERO);
        drawer.open();
        drawer.state.overlay_rect = Some(Rect::new(10, 5, 20, 10));

        assert!(!drawer.close_on_outside_click(Position::new(15, 8)));
        assert!(drawer.is_open());
        assert!(drawer.close_on_outside_click(Position::new(2, 8)));
        assert!(drawer.is_closed());
    }

    #[test]
    fn outside_click_without_rendered_drawer_does_nothing() {
        let mut drawer = Drawer::new();

        assert!(!drawer.close_on_outside_click(Position::new(2, 8)));
        assert!(drawer.is_closed());
    }

    #[test]
    fn drawer_panel_has_contrast_against_backdrop() {
        let mut terminal = Terminal::new(TestBackend::new(50, 20)).unwrap();
        let mut drawer = Drawer::new().with_duration(Duration::ZERO);
        drawer.open();

        terminal
            .draw(|frame| {
                DrawerContent::new().render(frame, frame.area(), &mut drawer);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(35, 5)].bg, DRAWER_BACKGROUND);
        assert_eq!(buffer[(5, 5)].bg, Color::Black);
    }
}
