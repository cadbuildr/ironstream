// FILE: b_rep_graph_copy_remap.rs
// occt: BRepGraph_CopyRemap

use std::collections::HashMap;

/// Distinguishes copy vs. compact migration semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyRemapMode {
    /// Full graph copy: source and target are distinct graphs.
    Copy = 0,
    /// In-place compaction: layers migrate into the same (rebuilt) graph.
    Compact = 1,
}

/// Distinguishes explicit item map vs. identity mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingKind {
    /// Use the item remap for source->target resolution.
    Explicit = 0,
    /// Source and target ids are identical (full identity copy).
    Identity = 1,
}

/// Immutable context passed to layer copy callbacks.
///
/// The structural copy algorithm owns remap construction. Layers receive this
/// context and decide how to copy their own representation without exposing layer
/// details back to BRepGraph_Copy.
pub struct BRepGraphCopyRemap {
    source_graph_id: usize,
    target_graph_id: usize,
    item_remap: HashMap<u32, u32>,
    copy_mode: CopyRemapMode,
    mapping_kind: MappingKind,
}

impl BRepGraphCopyRemap {
    /// Create a new copy remap with explicit item map.
    pub fn new(
        source_graph_id: usize,
        target_graph_id: usize,
        item_remap: HashMap<u32, u32>,
        copy_mode: CopyRemapMode,
    ) -> Self {
        Self {
            source_graph_id,
            target_graph_id,
            item_remap,
            copy_mode,
            mapping_kind: MappingKind::Explicit,
        }
    }

    /// Create a new copy remap with identity mapping.
    pub fn new_identity(
        source_graph_id: usize,
        target_graph_id: usize,
        copy_mode: CopyRemapMode,
    ) -> Self {
        Self {
            source_graph_id,
            target_graph_id,
            item_remap: HashMap::new(),
            copy_mode,
            mapping_kind: MappingKind::Identity,
        }
    }

    /// Migration mode of this context.
    pub fn copy_mode(&self) -> CopyRemapMode {
        self.copy_mode
    }

    /// True if this is a compaction migration (not a full copy).
    pub fn is_compact(&self) -> bool {
        self.copy_mode == CopyRemapMode::Compact
    }

    /// Source graph id.
    pub fn source_graph_id(&self) -> usize {
        self.source_graph_id
    }

    /// Target graph id.
    pub fn target_graph_id(&self) -> usize {
        self.target_graph_id
    }

    /// Source item id -> target item id map for copied definitions, refs, and reps.
    pub fn items(&self) -> &HashMap<u32, u32> {
        &self.item_remap
    }

    /// Return the target item for a source item, or 0 (invalid) if not copied.
    pub fn target_item(&self, source_item: u32) -> u32 {
        if source_item == 0 {
            return 0;
        }
        *self.item_remap.get(&source_item).unwrap_or(&0)
    }

    /// Return true if the source item has a valid copied target item.
    pub fn has_target_item(&self, source_item: u32) -> bool {
        if source_item == 0 {
            return false;
        }
        self.item_remap.contains_key(&source_item)
            && *self.item_remap.get(&source_item).unwrap_or(&0) != 0
    }

    /// Return the mapping kind.
    pub fn mapping_kind(&self) -> MappingKind {
        self.mapping_kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_remap_explicit() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);

        let remap = BRepGraphCopyRemap::new(1, 2, map, CopyRemapMode::Copy);

        assert_eq!(remap.copy_mode(), CopyRemapMode::Copy);
        assert!(!remap.is_compact());
        assert_eq!(remap.source_graph_id(), 1);
        assert_eq!(remap.target_graph_id(), 2);
        assert_eq!(remap.target_item(1), 10);
        assert_eq!(remap.target_item(2), 20);
        assert_eq!(remap.target_item(3), 0);
        assert!(remap.has_target_item(1));
        assert!(remap.has_target_item(2));
        assert!(!remap.has_target_item(3));
    }

    #[test]
    fn test_copy_remap_identity() {
        let remap = BRepGraphCopyRemap::new_identity(1, 2, CopyRemapMode::Compact);

        assert_eq!(remap.copy_mode(), CopyRemapMode::Compact);
        assert!(remap.is_compact());
        assert_eq!(remap.mapping_kind(), MappingKind::Identity);
    }

    #[test]
    fn test_copy_remap_invalid_item() {
        let remap = BRepGraphCopyRemap::new_identity(1, 2, CopyRemapMode::Copy);

        assert!(!remap.has_target_item(0));
        assert_eq!(remap.target_item(0), 0);
    }
}
