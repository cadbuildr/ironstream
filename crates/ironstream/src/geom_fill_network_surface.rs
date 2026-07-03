// FILE: geom_fill_network_surface.rs
// occt: GeomFill_NetworkSurface

/// Network surface filling
pub struct NetworkSurface {
    is_done: bool,
}

impl NetworkSurface {
    pub fn new() -> Self {
        NetworkSurface { is_done: false }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Default for NetworkSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let surf = NetworkSurface::new();
        assert!(!surf.is_done());
    }

    #[test]
    fn test_default() {
        let _surf = NetworkSurface::default();
    }
}
