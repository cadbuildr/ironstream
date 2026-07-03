// FILE: aspect_touch.rs
// occt: Aspect_Touch

/// Touch event data.
#[derive(Debug, Clone, Copy)]
pub struct AspectTouch {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

impl AspectTouch {
    /// Create touch event.
    pub fn new(id: i32, x: f32, y: f32, pressure: f32) -> Self {
        AspectTouch { id, x, y, pressure }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let touch = AspectTouch::new(1, 100.0, 200.0, 0.8);
        assert_eq!(touch.id, 1);
        assert_eq!(touch.x, 100.0);
        assert_eq!(touch.pressure, 0.8);
    }
}
