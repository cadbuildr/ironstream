// FILE: ais_view_input_buffer.rs
// occt: AIS_ViewInputBuffer

#[derive(Clone, Debug)]
pub struct AisViewInputBuffer {
    pub buffer_id: u32,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

impl AisViewInputBuffer {
    pub fn new(buffer_id: u32) -> Self {
        Self { buffer_id, mouse_x: 0.0, mouse_y: 0.0 }
    }
    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
}

impl Default for AisViewInputBuffer {
    fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_mouse_pos() {
        let mut buf = AisViewInputBuffer::new(1);
        buf.set_mouse_pos(100.0, 200.0);
        assert_eq!(buf.mouse_x, 100.0);
    }
}
