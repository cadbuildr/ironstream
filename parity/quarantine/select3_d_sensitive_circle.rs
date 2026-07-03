// FILE: select3_d_sensitive_circle.rs
// occt: Select3D_SensitiveCircle

/// Stub for Select3D_SensitiveCircle from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_SensitiveCircle {}

impl Select3D_SensitiveCircle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_SensitiveCircle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_sensitive_circle_creation() {
        let _obj = Select3D_SensitiveCircle::new();
        let _def = Select3D_SensitiveCircle::default();
    }
}
