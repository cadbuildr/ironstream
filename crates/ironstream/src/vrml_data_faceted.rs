// FILE: vrml_data_faceted.rs
// occt: VrmlData_Faceted

#[derive(Clone, Debug)]
pub struct VrmlDataFaceted {
    is_faceted: bool,
}

impl VrmlDataFaceted {
    pub fn new(is_faceted: bool) -> Self {
        VrmlDataFaceted { is_faceted }
    }

    pub fn is_faceted(&self) -> bool {
        self.is_faceted
    }

    pub fn set_faceted(&mut self, val: bool) {
        self.is_faceted = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let f = VrmlDataFaceted::new(true);
        assert!(f.is_faceted());
    }

    #[test]
    fn test_set_faceted() {
        let mut f = VrmlDataFaceted::new(false);
        f.set_faceted(true);
        assert!(f.is_faceted());
    }
}
