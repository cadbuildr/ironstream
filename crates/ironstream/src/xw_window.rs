// FILE: xw_window.rs
// occt: Xw_Window

#[derive(Clone, Debug)]
pub struct XwWindow {
    window_id: u64,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    display_name: String,
}

impl XwWindow {
    pub fn new(window_id: u64, width: u32, height: u32) -> Self {
        Self { window_id, width, height, x: 0, y: 0, display_name: String::new() }
    }
    pub fn window_id(&self) -> u64 { self.window_id }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn display_name(&self) -> &str { &self.display_name }
    pub fn set_position(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_display_name(&mut self, name: String) { self.display_name = name; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let w = XwWindow::new(0x12345678, 640, 480);
        assert_eq!(w.window_id(), 0x12345678);
    }
}
