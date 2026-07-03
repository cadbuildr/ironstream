// FILE: aspect_window_input_listener.rs
// occt: Aspect_WindowInputListener

use std::collections::HashMap;

/// Simple 2D integer vector for mouse position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
}

/// Simple 2D floating-point vector for touch position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2f { x, y }
    }
}

/// Simple 3D boolean vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3b {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Vec3b {
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Vec3b { x, y, z }
    }
}

/// Touch point information.
#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub position: Vec2f,
}

/// Virtual key set for keyboard state.
#[derive(Debug, Clone, Default)]
pub struct VKeySet {
    keys: HashMap<u32, bool>,
}

impl VKeySet {
    pub fn new() -> Self {
        VKeySet {
            keys: HashMap::new(),
        }
    }

    pub fn is_pressed(&self, key: u32) -> bool {
        self.keys.get(&key).copied().unwrap_or(false)
    }

    pub fn set_pressed(&mut self, key: u32, pressed: bool) {
        if pressed {
            self.keys.insert(key, true);
        } else {
            self.keys.remove(&key);
        }
    }
}

/// Scroll delta information.
#[derive(Debug, Clone)]
pub struct ScrollDelta {
    pub position: Vec2i,
    pub delta: i32,
}

/// Mouse button and key modifier flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VKeyMouse(u32);

impl VKeyMouse {
    pub fn new(value: u32) -> Self {
        VKeyMouse(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn has_flag(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    pub fn with_flag(&self, flag: u32) -> Self {
        VKeyMouse(self.0 | flag)
    }

    pub fn without_flag(&self, flag: u32) -> Self {
        VKeyMouse(self.0 & !flag)
    }
}

/// Key modifier flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VKeyFlags(u32);

impl VKeyFlags {
    pub fn new(value: u32) -> Self {
        VKeyFlags(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Base class for window input event listener.
/// Abstract listener interface for window input events.
pub struct AspectWindowInputListener {
    keys: VKeySet,
    mouse_position_last: Vec2i,
    mouse_pressed: VKeyMouse,
    mouse_modifiers: VKeyFlags,
    touch_points: HashMap<usize, TouchPoint>,
    mouse_3d_accel_trans: f32,
    mouse_3d_accel_rotate: f32,
    mouse_3d_is_quadric: bool,
    mouse_3d_no_rotate: Vec3b,
    mouse_3d_to_reverse: Vec3b,
    mouse_3d_button_state: [bool; 32],
}

impl AspectWindowInputListener {
    /// Create a new window input listener.
    pub fn new() -> Self {
        AspectWindowInputListener {
            keys: VKeySet::new(),
            mouse_position_last: Vec2i::new(0, 0),
            mouse_pressed: VKeyMouse::new(0),
            mouse_modifiers: VKeyFlags::new(0),
            touch_points: HashMap::new(),
            mouse_3d_accel_trans: 2.0,
            mouse_3d_accel_rotate: 4.0,
            mouse_3d_is_quadric: true,
            mouse_3d_no_rotate: Vec3b::new(false, false, false),
            mouse_3d_to_reverse: Vec3b::new(true, false, false),
            mouse_3d_button_state: [false; 32],
        }
    }

    /// Return keyboard state.
    pub fn keys(&self) -> &VKeySet {
        &self.keys
    }

    /// Return mutable keyboard state.
    pub fn keys_mut(&mut self) -> &mut VKeySet {
        &mut self.keys
    }

    /// Return currently pressed mouse buttons.
    pub fn pressed_mouse_buttons(&self) -> VKeyMouse {
        self.mouse_pressed
    }

    /// Return active key modifiers passed with last mouse event.
    pub fn last_mouse_flags(&self) -> VKeyFlags {
        self.mouse_modifiers
    }

    /// Return last mouse position.
    pub fn last_mouse_position(&self) -> Vec2i {
        self.mouse_position_last
    }

    /// Check if there are any active touch points.
    pub fn has_touch_points(&self) -> bool {
        !self.touch_points.is_empty()
    }

    /// Return map of active touches.
    pub fn touch_points(&self) -> &HashMap<usize, TouchPoint> {
        &self.touch_points
    }

    /// Add a touch point with the given ID.
    pub fn add_touch_point(&mut self, id: usize, position: Vec2f, clear_before: bool) {
        if clear_before {
            self.touch_points.clear();
        }
        self.touch_points.insert(id, TouchPoint { position });
    }

    /// Remove a touch point with the given ID.
    pub fn remove_touch_point(&mut self, id: usize, _clear_select_pnts: bool) -> bool {
        self.touch_points.remove(&id).is_some()
    }

    /// Update a touch point with the given ID.
    pub fn update_touch_point(&mut self, id: usize, position: Vec2f) {
        self.touch_points.insert(id, TouchPoint { position });
    }

    /// Return acceleration ratio for translation event (default 2.0).
    pub fn get_3d_mouse_translation_scale(&self) -> f32 {
        self.mouse_3d_accel_trans
    }

    /// Set acceleration ratio for translation event.
    pub fn set_3d_mouse_translation_scale(&mut self, scale: f32) {
        self.mouse_3d_accel_trans = scale;
    }

    /// Return acceleration ratio for rotation event (default 4.0).
    pub fn get_3d_mouse_rotation_scale(&self) -> f32 {
        self.mouse_3d_accel_rotate
    }

    /// Set acceleration ratio for rotation event.
    pub fn set_3d_mouse_rotation_scale(&mut self, scale: f32) {
        self.mouse_3d_accel_rotate = scale;
    }

    /// Return quadric acceleration flag (default true).
    pub fn to_3d_mouse_precise_input(&self) -> bool {
        self.mouse_3d_is_quadric
    }

    /// Set quadric acceleration flag.
    pub fn set_3d_mouse_precise_input(&mut self, is_quadric: bool) {
        self.mouse_3d_is_quadric = is_quadric;
    }

    /// Return 3d mouse rotation axes ignore flag.
    pub fn get_3d_mouse_is_no_rotate(&self) -> Vec3b {
        self.mouse_3d_no_rotate
    }

    /// Return mutable 3d mouse rotation axes ignore flag.
    pub fn change_3d_mouse_is_no_rotate(&mut self) -> &mut Vec3b {
        &mut self.mouse_3d_no_rotate
    }

    /// Return 3d mouse rotation axes reverse flag.
    pub fn get_3d_mouse_to_reverse(&self) -> Vec3b {
        self.mouse_3d_to_reverse
    }

    /// Return mutable 3d mouse rotation axes reverse flag.
    pub fn change_3d_mouse_to_reverse(&mut self) -> &mut Vec3b {
        &mut self.mouse_3d_to_reverse
    }

    /// Handle mouse button press event.
    pub fn press_mouse_button(&mut self, position: Vec2i, button: VKeyMouse, modifiers: VKeyFlags, is_emulated: bool) -> bool {
        let new_buttons = self.mouse_pressed.with_flag(button.value());
        self.update_mouse_buttons(position, new_buttons, modifiers, is_emulated)
    }

    /// Handle mouse button release event.
    pub fn release_mouse_button(&mut self, position: Vec2i, button: VKeyMouse, modifiers: VKeyFlags, is_emulated: bool) -> bool {
        let new_buttons = self.mouse_pressed.without_flag(button.value());
        self.update_mouse_buttons(position, new_buttons, modifiers, is_emulated)
    }

    /// Update mouse position and button state.
    pub fn update_mouse_buttons(&mut self, position: Vec2i, buttons: VKeyMouse, modifiers: VKeyFlags, _is_emulated: bool) -> bool {
        self.mouse_position_last = position;
        self.mouse_pressed = buttons;
        self.mouse_modifiers = modifiers;
        true
    }

    /// Update mouse position.
    pub fn update_mouse_position(&mut self, position: Vec2i, buttons: VKeyMouse, modifiers: VKeyFlags, _is_emulated: bool) -> bool {
        self.mouse_position_last = position;
        self.mouse_pressed = buttons;
        self.mouse_modifiers = modifiers;
        true
    }

    /// Update mouse scroll event.
    pub fn update_mouse_scroll(&mut self, delta: &ScrollDelta) -> bool {
        self.mouse_position_last = delta.position;
        true
    }
}

impl Default for AspectWindowInputListener {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listener_creation() {
        let listener = AspectWindowInputListener::new();
        assert!(!listener.has_touch_points());
        assert_eq!(listener.get_3d_mouse_translation_scale(), 2.0);
        assert_eq!(listener.get_3d_mouse_rotation_scale(), 4.0);
        assert!(listener.to_3d_mouse_precise_input());
    }

    #[test]
    fn test_touch_point_operations() {
        let mut listener = AspectWindowInputListener::new();
        assert!(!listener.has_touch_points());

        listener.add_touch_point(0, Vec2f::new(10.0, 20.0), false);
        assert!(listener.has_touch_points());

        let removed = listener.remove_touch_point(0, false);
        assert!(removed);
        assert!(!listener.has_touch_points());
    }

    #[test]
    fn test_mouse_button_operations() {
        let mut listener = AspectWindowInputListener::new();
        let pos = Vec2i::new(100, 200);
        let button = VKeyMouse::new(1);
        let modifiers = VKeyFlags::new(0);

        listener.press_mouse_button(pos, button, modifiers, false);
        assert_eq!(listener.pressed_mouse_buttons().value(), 1);

        listener.release_mouse_button(pos, button, modifiers, false);
        assert_eq!(listener.pressed_mouse_buttons().value(), 0);
    }

    #[test]
    fn test_3d_mouse_scales() {
        let mut listener = AspectWindowInputListener::new();

        listener.set_3d_mouse_translation_scale(5.0);
        assert_eq!(listener.get_3d_mouse_translation_scale(), 5.0);

        listener.set_3d_mouse_rotation_scale(10.0);
        assert_eq!(listener.get_3d_mouse_rotation_scale(), 10.0);
    }

    #[test]
    fn test_mouse_position_tracking() {
        let mut listener = AspectWindowInputListener::new();
        let pos = Vec2i::new(50, 75);

        listener.update_mouse_position(pos, VKeyMouse::new(0), VKeyFlags::new(0), false);
        assert_eq!(listener.last_mouse_position(), pos);
    }
}
