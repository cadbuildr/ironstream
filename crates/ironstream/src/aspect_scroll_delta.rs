// FILE: aspect_scroll_delta.rs
// occt: Aspect_ScrollDelta

/// Scroll wheel delta.
#[derive(Debug, Clone, Copy)]
pub struct AspectScrollDelta {
    pub delta_x: f32,
    pub delta_y: f32,
}

impl AspectScrollDelta {
    /// Create scroll delta.
    pub fn new(delta_x: f32, delta_y: f32) -> Self {
        AspectScrollDelta { delta_x, delta_y }
    }

    /// Create vertical scroll.
    pub fn vertical(delta: f32) -> Self {
        AspectScrollDelta {
            delta_x: 0.0,
            delta_y: delta,
        }
    }

    /// Create horizontal scroll.
    pub fn horizontal(delta: f32) -> Self {
        AspectScrollDelta {
            delta_x: delta,
            delta_y: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scroll = AspectScrollDelta::new(1.0, -2.0);
        assert_eq!(scroll.delta_x, 1.0);
        assert_eq!(scroll.delta_y, -2.0);
    }

    #[test]
    fn test_vertical() {
        let scroll = AspectScrollDelta::vertical(3.0);
        assert_eq!(scroll.delta_x, 0.0);
        assert_eq!(scroll.delta_y, 3.0);
    }
}
