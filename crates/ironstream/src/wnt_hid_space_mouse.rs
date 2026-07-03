// FILE: wnt_hid_space_mouse.rs
// occt: WNT_HIDSpaceMouse

#[derive(Clone, Debug, Default)]
pub struct HidSpaceMouse {
    device_id: u32,
    translation: [f32; 3],
    rotation: [f32; 3],
    is_connected: bool,
}

impl HidSpaceMouse {
    pub fn new() -> Self {
        Self { device_id: 0, translation: [0.0; 3], rotation: [0.0; 3], is_connected: false }
    }
    pub fn device_id(&self) -> u32 { self.device_id }
    pub fn translation(&self) -> [f32; 3] { self.translation }
    pub fn rotation(&self) -> [f32; 3] { self.rotation }
    pub fn is_connected(&self) -> bool { self.is_connected }
    pub fn set_device_id(&mut self, id: u32) { self.device_id = id; }
    pub fn set_translation(&mut self, t: [f32; 3]) { self.translation = t; }
    pub fn set_rotation(&mut self, r: [f32; 3]) { self.rotation = r; }
    pub fn set_connected(&mut self, connected: bool) { self.is_connected = connected; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = HidSpaceMouse::new();
        assert_eq!(m.device_id(), 0);
    }
}
