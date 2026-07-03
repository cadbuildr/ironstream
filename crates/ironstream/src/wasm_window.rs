// FILE: wasm_window.rs
// occt: Wasm_Window

#[derive(Clone, Debug)]
pub struct WasmWindow {
    canvas_id: String,
    width: u32,
    height: u32,
    device_pixel_ratio: f32,
}

impl WasmWindow {
    pub fn new(canvas_id: String, width: u32, height: u32) -> Self {
        Self { canvas_id, width, height, device_pixel_ratio: 1.0 }
    }
    pub fn canvas_id(&self) -> &str { &self.canvas_id }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn device_pixel_ratio(&self) -> f32 { self.device_pixel_ratio }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_device_pixel_ratio(&mut self, ratio: f32) { self.device_pixel_ratio = ratio.max(0.1); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let w = WasmWindow::new("canvas".to_string(), 1024, 768);
        assert_eq!(w.canvas_id(), "canvas");
    }
}
