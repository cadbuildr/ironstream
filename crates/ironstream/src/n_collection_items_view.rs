// FILE: n_collection_items_view.rs
// occt: NCollection_ItemsView // | KeyValueRef | KeyValueIndexRef | KeyIndexRef | Iterator | View

/// Key-value pair reference for structured binding support.
/// Enables: for [key, value] in map.items()
pub struct KeyValueRef<K, V> {
    pub key: K,
    pub value: V,
}

/// Key-value-index tuple reference for structured binding support.
/// Enables: for [key, value, index] in map.indexed_items()
pub struct KeyValueIndexRef<K, V> {
    pub key: K,
    pub value: V,
    pub index: i32,
}

/// Key-index pair reference for structured binding support (key-only indexed maps).
/// Enables: for [key, index] in map.indexed_items()
pub struct KeyIndexRef<K> {
    pub key: K,
    pub index: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_ref_creation() {
        let kv = KeyValueRef { key: 42, value: "hello" };
        assert_eq!(kv.key, 42);
        assert_eq!(kv.value, "hello");
    }

    #[test]
    fn test_key_value_index_ref_creation() {
        let kvi = KeyValueIndexRef {
            key: "key",
            value: 100.5,
            index: 0,
        };
        assert_eq!(kvi.key, "key");
        assert_eq!(kvi.value, 100.5);
        assert_eq!(kvi.index, 0);
    }

    #[test]
    fn test_key_index_ref_creation() {
        let ki = KeyIndexRef { key: 7, index: 3 };
        assert_eq!(ki.key, 7);
        assert_eq!(ki.index, 3);
    }
}
