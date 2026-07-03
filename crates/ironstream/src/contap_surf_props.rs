// FILE: contap_surf_props.rs
// occt: Contap_SurfProps

pub struct ContapSurfProps;

impl ContapSurfProps {
    pub fn new() -> Self {
        ContapSurfProps
    }
}

impl Default for ContapSurfProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapSurfProps;

    #[test]
    fn test_new() {
        let _ = ContapSurfProps::new();
    }
}
