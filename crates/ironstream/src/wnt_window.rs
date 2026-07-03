// FILE: wnt_window.rs
// occt: WNT_Window

#[derive(Clone, Debug)]
pub struct Window {
    hwnd: usize,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    is_visible: bool,
}

impl Window {
    pub fn new(hwnd: usize, width: u32, height: u32) -> Self {
        Self { hwnd, width, height, x: 0, y: 0, is_visible: true }
    }
    pub fn hwnd(&self) -> usize { self.hwnd }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn is_visible(&self) -> bool { self.is_visible }
    pub fn set_position(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
    pub fn set_size(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let w = Window::new(0x12345678, 800, 600);
        assert_eq!(w.hwnd(), 0x12345678);
    }
}
