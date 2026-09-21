use std::time::Duration;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Gauge, Paragraph, Wrap};

use crate::button::{Button, ButtonVariant};
use crate::tui_overlay::anchor::Anchor;
use crate::tui_overlay::backdrop::Backdrop;
use crate::tui_overlay::overlay::Overlay;
use crate::tui_overlay::slide::Slide;
use crate::tui_overlay::state::OverlayState;

const ANIMATION_TIME: Duration = Duration::from_millis(160);
const DEFAULT_DURATION: Duration = Duration::from_secs(5);

/// Owns the visibility and animation state of a composed toast.
#[derive(Debug, Clone)]
pub struct Toast {
    state: OverlayState,
    duration: Option<Duration>,
    remaining: Option<Duration>,
    dismissible: bool,
}

impl Toast {
    pub fn new() -> Self {
        Self {
            state: OverlayState::new().with_duration(ANIMATION_TIME),
            duration: Some(DEFAULT_DURATION),
            remaining: Some(DEFAULT_DURATION),
            dismissible: true,
        }
    }

    /// Set auto-dismiss duration. Use `Duration::ZERO` to disable the timer.
    pub fn duration(mut self, duration: Duration) -> Self {
        let duration = (!duration.is_zero()).then_some(duration);
        self.duration = duration;
        self.remaining = duration;
        self
    }

    /// Disable manual dismissal while keeping auto-dismiss enabled.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn open(&mut self) {
        self.remaining = self.duration;
        self.state.open();
    }

    pub fn close(&mut self) {
        if self.dismissible {
            self.state.close();
        }
    }

    /// Close when a click lands outside the rendered toast. Returns whether dismissal was requested.
    pub fn close_on_outside_click(&mut self, position: Position) -> bool {
        if !self.dismissible {
            return false;
        }
        let Some(rect) = self.state.overlay_rect() else {
            return false;
        };
        if rect.contains(position) {
            return false;
        }
        self.close();
        true
    }

    pub fn tick(&mut self, elapsed: Duration) {
        self.state.tick(elapsed);
        if self.state.is_closed() {
            return;
        }

        let expired = self.remaining.as_mut().is_some_and(|remaining| {
            *remaining = remaining.saturating_sub(elapsed);
            remaining.is_zero()
        });
        if expired {
            self.state.close();
        }
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

    pub fn is_dismissible(&self) -> bool {
        self.dismissible
    }

    /// Remaining lifetime as a fraction from 1.0 to 0.0; `None` when timer disabled.
    pub fn remaining_ratio(&self) -> Option<f64> {
        let duration = self.duration?;
        let remaining = self.remaining?;
        Some((remaining.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0))
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self::new()
    }
}

/// Button that opens a parent-owned [`Toast`] when activated.
#[must_use]
pub struct ToastTrigger<'a> {
    button: Button<'a>,
}

impl<'a> ToastTrigger<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            button: Button::new(label).variant(ButtonVariant::Default),
        }
    }

    pub fn activate(&self, toast: &mut Toast) {
        toast.open();
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

/// Animated bottom-right surface. Render child components into returned areas.
#[must_use]
pub struct ToastContent {
    width: Constraint,
    height: Constraint,
    variant: ToastVariant,
}

/// Style for a toast surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
}

impl ToastVariant {
    fn accent(self) -> Color {
        match self {
            Self::Default => Color::Rgb(82, 82, 91),
            Self::Success => Color::Rgb(34, 197, 94),
            Self::Info => Color::Rgb(59, 130, 246),
            Self::Warning => Color::Rgb(245, 158, 11),
            Self::Error => Color::Rgb(239, 68, 68),
        }
    }
}

/// Regions exposed by [`ToastContent`] for composing toast children.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToastAreas {
    pub outer: Rect,
    pub title: Rect,
    pub description: Rect,
    pub tracker: Option<Rect>,
    pub close: Option<Rect>,
}

impl ToastContent {
    pub fn new() -> Self {
        Self {
            width: Constraint::Length(56),
            height: Constraint::Length(6),
            variant: ToastVariant::default(),
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

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn render(
        self,
        frame: &mut Frame<'_>,
        area: Rect,
        toast: &mut Toast,
    ) -> Option<ToastAreas> {
        let border = Style::default().fg(self.variant.accent());
        let overlay = Overlay::new()
            .anchor(Anchor::BottomRight)
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
        frame.render_stateful_widget(overlay, area, &mut toast.state);

        let outer = toast.state.overlay_rect()?;
        let inner = toast.state.inner_area()?;
        let tracker_rows = u16::from(toast.duration.is_some());
        let [title, description, tracker] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(tracker_rows),
        ])
        .areas(inner);
        let close = toast.dismissible.then(|| {
            let close_width = 5.min(title.width);
            Rect::new(
                title.right().saturating_sub(close_width),
                title.y,
                close_width,
                title.height,
            )
        });
        let title = Rect::new(
            title.x,
            title.y,
            title
                .width
                .saturating_sub(close.map_or(0, |area| area.width)),
            title.height,
        );

        Some(ToastAreas {
            outer,
            title,
            description,
            tracker: toast.duration.map(|_| tracker),
            close,
        })
    }
}

impl Default for ToastContent {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress bar that drains as the parent [`Toast`] approaches auto-dismiss.
#[must_use]
pub struct ToastTracker {
    variant: ToastVariant,
}

impl ToastTracker {
    pub fn new() -> Self {
        Self {
            variant: ToastVariant::default(),
        }
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect, toast: &Toast) {
        let Some(ratio) = toast.remaining_ratio() else {
            return;
        };
        frame.render_widget(
            Gauge::default()
                .ratio(ratio)
                .label("")
                .gauge_style(Style::default().fg(self.variant.accent()))
                .style(Style::default().bg(Color::Rgb(39, 39, 42))),
            area,
        );
    }
}

impl Default for ToastTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Bold title for a composed toast.
#[must_use]
pub struct ToastTitle<'a> {
    text: &'a str,
    variant: ToastVariant,
}

impl<'a> ToastTitle<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            variant: ToastVariant::default(),
        }
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        let color = match self.variant {
            ToastVariant::Default => Color::Rgb(250, 250, 250),
            variant => variant.accent(),
        };
        frame.render_widget(
            Paragraph::new(Line::styled(
                self.text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )),
            area,
        );
    }
}

/// Supporting message for a composed toast.
#[must_use]
pub struct ToastDescription<'a> {
    text: &'a str,
}

impl<'a> ToastDescription<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.text)
                .style(Style::default().fg(Color::Rgb(161, 161, 170)))
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

/// Button that closes a parent-owned [`Toast`].
#[must_use]
pub struct ToastClose<'a> {
    button: Button<'a>,
}

impl<'a> ToastClose<'a> {
    pub fn new() -> Self {
        Self {
            button: Button::new("x").variant(ButtonVariant::Ghost),
        }
    }

    pub fn activate(&self, toast: &mut Toast) {
        toast.close();
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.button = self.button.focused(focused);
        self
    }

    pub fn render(self, frame: &mut Frame<'_>, area: Rect) {
        self.button.render(frame, area);
    }
}

impl Default for ToastClose<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracker_ratio_decreases_with_elapsed_time() {
        let mut toast = Toast::new().duration(Duration::from_secs(2));
        toast.open();
        toast.tick(Duration::from_secs(1));

        assert_eq!(toast.remaining_ratio(), Some(0.5));
    }

    #[test]
    fn non_dismissible_toast_ignores_manual_close_but_times_out() {
        let mut toast = Toast::new()
            .duration(Duration::from_millis(200))
            .dismissible(false);
        toast.open();
        toast.tick(ANIMATION_TIME);
        toast.close();
        assert!(toast.is_open());

        toast.tick(Duration::from_millis(40));
        assert!(toast.is_animating());
        toast.tick(ANIMATION_TIME);
        assert!(toast.is_closed());
    }

    #[test]
    fn outside_click_dismisses_only_when_outside_rendered_toast() {
        let mut toast = Toast::new();
        toast.state = OverlayState::new();
        toast.open();
        toast.state.overlay_rect = Some(Rect::new(10, 5, 20, 6));

        assert!(!toast.close_on_outside_click(Position::new(12, 7)));
        assert!(toast.is_open());
        assert!(toast.close_on_outside_click(Position::new(2, 7)));
        assert!(toast.is_closed());
    }

    #[test]
    fn outside_click_does_not_dismiss_non_dismissible_or_unrendered_toast() {
        let position = Position::new(2, 7);
        let mut toast = Toast::new().dismissible(false);
        toast.state = OverlayState::new();
        toast.open();
        toast.state.overlay_rect = Some(Rect::new(10, 5, 20, 6));

        assert!(!toast.close_on_outside_click(position));
        assert!(toast.is_open());

        let mut unrendered = Toast::new();
        unrendered.state = OverlayState::new();
        unrendered.open();
        assert!(!unrendered.close_on_outside_click(position));
        assert!(unrendered.is_open());
    }

    #[test]
    fn zero_duration_disables_tracker_and_auto_dismiss() {
        let mut toast = Toast::new().duration(Duration::ZERO);
        toast.open();
        toast.tick(ANIMATION_TIME);
        toast.tick(Duration::from_secs(60));

        assert!(toast.is_open());
        assert_eq!(toast.remaining_ratio(), None);
    }
}
