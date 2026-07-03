// FILE: n_collection_define_hasher.rs
// occt: NCollection_DefineHasher

/// Macro-like hasher definition for collections.
pub struct NCollectionDefineHasher;

impl NCollectionDefineHasher {
    pub fn create_default_hasher() -> NCollectionDefineHasher {
        NCollectionDefineHasher
    }

    pub fn hash_value(&self, _value: i32) -> u32 {
        0 // Stub
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _hasher = NCollectionDefineHasher::create_default_hasher();
    }
}
