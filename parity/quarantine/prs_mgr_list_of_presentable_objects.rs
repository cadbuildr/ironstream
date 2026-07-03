// FILE: prs_mgr_list_of_presentable_objects.rs
// occt: PrsMgr_ListOfPresentableObjects

/// Stub for PrsMgr_ListOfPresentableObjects from OCCT.
#[derive(Clone, Debug)]
pub struct PrsMgr_ListOfPresentableObjects {}

impl PrsMgr_ListOfPresentableObjects {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrsMgr_ListOfPresentableObjects {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs_mgr_list_of_presentable_objects_creation() {
        let _obj = PrsMgr_ListOfPresentableObjects::new();
        let _def = PrsMgr_ListOfPresentableObjects::default();
    }
}
