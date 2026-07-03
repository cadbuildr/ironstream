// FILE: shape_custom_convert_to_revolution.rs
// occt: ShapeCustom_ConvertToRevolution

/// Converts surfaces to surfaces of revolution.
pub struct ConvertToRevolution {
    data: Vec<u8>,
}

impl ConvertToRevolution {
    /// Create new converter.
    pub fn new() -> Self {
        ConvertToRevolution { data: Vec::new() }
    }
}

impl Default for ConvertToRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let conv = ConvertToRevolution::new();
        assert_eq!(conv.data.len(), 0);
    }
}
