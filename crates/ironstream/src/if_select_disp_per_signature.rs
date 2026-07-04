// FILE: if_select_disp_per_signature.rs
// occt: IFSelect_DispPerSignature

#[derive(Clone, Debug)]
pub struct IfSelectDispPerSignature;

impl IfSelectDispPerSignature {
    pub fn new() -> Self {
        IfSelectDispPerSignature
    }
}

impl Default for IfSelectDispPerSignature {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectDispPerSignature::new();
    }
}
