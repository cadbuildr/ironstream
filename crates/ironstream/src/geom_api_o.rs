// FILE: geom_api_o.rs
// occt-ref: GeomAPI

/// Geometry API package providing high-level interface for geometry operations
pub struct GeomAPI;

impl GeomAPI {
    pub fn new() -> Self {
        GeomAPI
    }
}

impl Default for GeomAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _api = GeomAPI::new();
    }

    #[test]
    fn test_default() {
        let _api = GeomAPI::default();
    }
}
