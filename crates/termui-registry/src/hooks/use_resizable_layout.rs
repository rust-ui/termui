use ratzilla::{
    event::{KeyCode, MouseButton, MouseEvent, MouseEventKind},
    ratatui::layout::{Position, Rect},
};

const INITIAL_PERCENT: u16 = 32;
const MIN_PERCENT: u16 = 15;
const MAX_PERCENT: u16 = 85;
const KEYBOARD_STEP: u16 = 2;

#[derive(Clone, Copy, strum::IntoStaticStr)]
#[strum(serialize_all = "PascalCase")]
pub(super) enum InteractionState {
    Idle,
    Active,
    Dragging,
}

pub(super) struct ResizableLayoutState {
    first_percent: u16,
    active: bool,
    dragging: bool,
    panels_area: Rect,
    handle_area: Rect,
}

impl ResizableLayoutState {
    fn new() -> Self {
        Self {
            first_percent: INITIAL_PERCENT,
            active: false,
            dragging: false,
            panels_area: Rect::default(),
            handle_area: Rect::default(),
        }
    }

    pub(super) fn first_percent(&self) -> u16 {
        self.first_percent
    }

    pub(super) fn interaction_state(&self) -> InteractionState {
        if self.dragging {
            InteractionState::Dragging
        } else if self.active {
            InteractionState::Active
        } else {
            InteractionState::Idle
        }
    }

    pub(super) fn is_highlighted(&self) -> bool {
        self.active || self.dragging
    }

    pub(super) fn set_hit_areas(&mut self, panels_area: Rect, handle_area: Rect) {
        self.panels_area = panels_area;
        self.handle_area = handle_area;
    }

    pub(super) fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => self.active = !self.active,
            KeyCode::Esc => self.active = false,
            KeyCode::Left if self.active => {
                self.first_percent = self
                    .first_percent
                    .saturating_sub(KEYBOARD_STEP)
                    .max(MIN_PERCENT);
            }
            KeyCode::Right if self.active => {
                self.first_percent = (self.first_percent + KEYBOARD_STEP).min(MAX_PERCENT);
            }
            _ => {}
        }
    }

    pub(super) fn handle_mouse(&mut self, event: MouseEvent) {
        let point = Position::new(event.col, event.row);
        match event.kind {
            MouseEventKind::Moved => {
                self.active = self.dragging || self.handle_area.contains(point);
                if self.dragging {
                    self.set_from_pointer(event.col);
                }
            }
            MouseEventKind::Exited => {
                self.active = false;
                self.dragging = false;
            }
            MouseEventKind::ButtonDown(MouseButton::Left) if self.handle_area.contains(point) => {
                self.active = true;
                self.dragging = true;
                self.set_from_pointer(event.col);
            }
            MouseEventKind::ButtonUp(MouseButton::Left) => {
                self.dragging = false;
                self.active = self.handle_area.contains(point);
            }
            _ => {}
        }
    }

    fn set_from_pointer(&mut self, column: u16) {
        let panels_width = self
            .panels_area
            .width
            .saturating_sub(self.handle_area.width);
        if panels_width == 0 {
            return;
        }

        let offset = column.saturating_sub(self.panels_area.x).min(panels_width);
        self.first_percent = (u32::from(offset) * 100 / u32::from(panels_width))
            .clamp(MIN_PERCENT.into(), MAX_PERCENT.into()) as u16;
    }
}

pub(super) fn use_resizable_layout() -> ResizableLayoutState {
    ResizableLayoutState::new()
}
