// FILE: d_data_std_draw_driver.rs
// occt: DDataStd_DrawDriver

//! Draw driver for DDataStd attributes.

/// DDataStd_DrawDriver: draw presentation driver.
#[derive(Clone, Debug)]
pub struct DDataStdDrawDriver {
    id: u32,
}

impl DDataStdDrawDriver {
    /// Create a new draw driver.
    pub fn new(id: u32) -> Self {
        DDataStdDrawDriver { id }
    }

    /// Draw the attribute.
    pub fn draw(&self) {
        // Render attribute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = DDataStdDrawDriver::new(1);
        assert_eq!(driver.id, 1);
    }
}
