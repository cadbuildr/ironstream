// FILE: select3_d_pnt.rs
// occt: Select3D_Pnt

/// Stub for Select3D_Pnt from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_Pnt {}

impl Select3D_Pnt {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_Pnt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_pnt_creation() {
        let _obj = Select3D_Pnt::new();
        let _def = Select3D_Pnt::default();
    }
}
