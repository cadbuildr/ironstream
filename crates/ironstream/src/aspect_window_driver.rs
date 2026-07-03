// FILE: aspect_window_driver.rs
// occt: Aspect_WindowDriver, Aspect_DisplayConnection, Aspect_VKeySet

/// Key code for virtual keyboard events.
/// occt: Aspect_VKey
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AspectVKey {
    #[default]
    Unknown,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Digit0, Digit1, Digit2, Digit3, Digit4,
    Digit5, Digit6, Digit7, Digit8, Digit9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Left, Right, Up, Down, Home, End, PageUp, PageDown,
    Space, Enter, Escape, Tab, Backspace, Delete, Insert,
    Shift, Control, Alt, Meta,
}

impl AspectVKey {
    pub fn is_modifier(&self) -> bool {
        matches!(self, Self::Shift | Self::Control | Self::Alt | Self::Meta)
    }

    pub fn is_arrow(&self) -> bool {
        matches!(self, Self::Left | Self::Right | Self::Up | Self::Down)
    }

    pub fn is_digit(&self) -> bool {
        matches!(self,
            Self::Digit0 | Self::Digit1 | Self::Digit2 | Self::Digit3 | Self::Digit4 |
            Self::Digit5 | Self::Digit6 | Self::Digit7 | Self::Digit8 | Self::Digit9
        )
    }
}

/// State bitmask for held modifier keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AspectKeyFlags(pub u8);

impl AspectKeyFlags {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1);
    pub const CTRL: Self = Self(2);
    pub const ALT: Self = Self(4);
    pub const META: Self = Self(8);

    pub fn has_shift(&self) -> bool { self.0 & 1 != 0 }
    pub fn has_ctrl(&self)  -> bool { self.0 & 2 != 0 }
    pub fn has_alt(&self)   -> bool { self.0 & 4 != 0 }
    pub fn has_meta(&self)  -> bool { self.0 & 8 != 0 }

    pub fn with_shift(self) -> Self { Self(self.0 | 1) }
    pub fn with_ctrl(self)  -> Self { Self(self.0 | 2) }
    pub fn with_alt(self)   -> Self { Self(self.0 | 4) }
}

/// Keyboard event (key + modifier state).
/// occt: Aspect_VKeySet
#[derive(Clone, Debug, Default)]
pub struct AspectVKeySet {
    pub pressed: Vec<AspectVKey>,
    pub flags: AspectKeyFlags,
}

impl AspectVKeySet {
    pub fn new() -> Self { Self::default() }

    pub fn press(&mut self, key: AspectVKey) {
        if !self.pressed.contains(&key) { self.pressed.push(key); }
        match key {
            AspectVKey::Shift   => self.flags = AspectKeyFlags(self.flags.0 | 1),
            AspectVKey::Control => self.flags = AspectKeyFlags(self.flags.0 | 2),
            AspectVKey::Alt     => self.flags = AspectKeyFlags(self.flags.0 | 4),
            _ => {}
        }
    }

    pub fn release(&mut self, key: AspectVKey) {
        self.pressed.retain(|k| *k != key);
        match key {
            AspectVKey::Shift   => self.flags = AspectKeyFlags(self.flags.0 & !1),
            AspectVKey::Control => self.flags = AspectKeyFlags(self.flags.0 & !2),
            AspectVKey::Alt     => self.flags = AspectKeyFlags(self.flags.0 & !4),
            _ => {}
        }
    }

    pub fn is_pressed(&self, key: AspectVKey) -> bool { self.pressed.contains(&key) }
    pub fn nb_pressed(&self) -> usize { self.pressed.len() }
    pub fn clear(&mut self) { self.pressed.clear(); self.flags = AspectKeyFlags::NONE; }
}

/// Display connection information.
/// occt: Aspect_DisplayConnection
#[derive(Clone, Debug)]
pub struct AspectDisplayConnection {
    pub display_name: String,
    pub screen: u32,
    pub is_connected: bool,
    pub width_px: u32,
    pub height_px: u32,
    pub dpi: f32,
}

impl Default for AspectDisplayConnection {
    fn default() -> Self {
        Self {
            display_name: ":0".into(),
            screen: 0,
            is_connected: false,
            width_px: 1920,
            height_px: 1080,
            dpi: 96.0,
        }
    }
}

impl AspectDisplayConnection {
    pub fn new() -> Self { Self::default() }

    pub fn connect(&mut self, display: impl Into<String>) {
        self.display_name = display.into();
        self.is_connected = true;
    }

    pub fn disconnect(&mut self) { self.is_connected = false; }
    pub fn is_connected(&self) -> bool { self.is_connected }

    pub fn physical_size_mm(&self) -> (f32, f32) {
        let w = self.width_px as f32 / self.dpi * 25.4;
        let h = self.height_px as f32 / self.dpi * 25.4;
        (w, h)
    }
}

/// Abstract window driver stub.
/// occt: Aspect_WindowDriver
#[derive(Clone, Debug)]
pub struct AspectWindowDriver {
    pub display: AspectDisplayConnection,
    pub key_state: AspectVKeySet,
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub is_visible: bool,
    pub is_fullscreen: bool,
    pub position: [i32; 2],
}

impl Default for AspectWindowDriver {
    fn default() -> Self {
        Self {
            display: AspectDisplayConnection::default(),
            key_state: AspectVKeySet::new(),
            width: 800,
            height: 600,
            title: String::new(),
            is_visible: false,
            is_fullscreen: false,
            position: [0, 0],
        }
    }
}

impl AspectWindowDriver {
    pub fn new(width: u32, height: u32, title: impl Into<String>) -> Self {
        Self { width, height, title: title.into(), ..Self::default() }
    }

    pub fn show(&mut self) { self.is_visible = true; }
    pub fn hide(&mut self) { self.is_visible = false; }
    pub fn set_fullscreen(&mut self, v: bool) { self.is_fullscreen = v; }

    pub fn resize(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn move_to(&mut self, x: i32, y: i32) { self.position = [x, y]; }

    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 { 1.0 } else { self.width as f32 / self.height as f32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vkey_classification() {
        assert!(AspectVKey::Shift.is_modifier());
        assert!(AspectVKey::Left.is_arrow());
        assert!(AspectVKey::Digit5.is_digit());
        assert!(!AspectVKey::A.is_modifier());
    }

    #[test]
    fn key_flags_composition() {
        let f = AspectKeyFlags::NONE.with_shift().with_ctrl();
        assert!(f.has_shift());
        assert!(f.has_ctrl());
        assert!(!f.has_alt());
    }

    #[test]
    fn vkey_set_press_release() {
        let mut ks = AspectVKeySet::new();
        ks.press(AspectVKey::A);
        ks.press(AspectVKey::Shift);
        assert!(ks.is_pressed(AspectVKey::A));
        assert!(ks.flags.has_shift());
        ks.release(AspectVKey::Shift);
        assert!(!ks.flags.has_shift());
        ks.clear();
        assert_eq!(ks.nb_pressed(), 0);
    }

    #[test]
    fn display_connection() {
        let mut dc = AspectDisplayConnection::new();
        assert!(!dc.is_connected());
        dc.connect(":0.0");
        assert!(dc.is_connected());
        let (w, h) = dc.physical_size_mm();
        assert!(w > 0.0);
        assert!(h > 0.0);
    }

    #[test]
    fn window_driver_resize() {
        let mut wd = AspectWindowDriver::new(800, 600, "Test");
        wd.show();
        assert!(wd.is_visible);
        assert!((wd.aspect_ratio() - 800.0 / 600.0).abs() < 1e-4);
        wd.resize(1920, 1080);
        assert_eq!(wd.width, 1920);
    }

    #[test]
    fn window_driver_fullscreen() {
        let mut wd = AspectWindowDriver::new(1920, 1080, "Full");
        wd.set_fullscreen(true);
        assert!(wd.is_fullscreen);
    }
}
