// FILE: n_collection_define_alloc.rs
// occt: NCollection_DefineAlloc

/// Macro-like allocator definition for collections.
pub struct NCollectionDefineAlloc;

impl NCollectionDefineAlloc {
    pub fn create_default_alloc() -> Box<dyn std::any::Any> {
        Box::new(vec![0u8; 0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_alloc() {
        let _alloc = NCollectionDefineAlloc::create_default_alloc();
    }
}
