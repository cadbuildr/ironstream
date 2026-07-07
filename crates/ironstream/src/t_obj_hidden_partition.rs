// FILE: t_obj_hidden_partition.rs
// occt: TObj_HiddenPartition

//! This class is a partition with predefined hidden flag.
//! Faithful port of `TObj_HiddenPartition` (.hxx + .cxx): a
//! TObj_Partition whose `GetTypeFlags()` returns all flags of the father
//! class except `Visible` (bit 0x0001), and which registers itself for
//! TObj persistence (DECLARE/IMPLEMENT_TOBJOCAF_PERSISTENCE) under the
//! name "TObj_HiddenPartition". Partition/label plumbing is local.

/// TObj_Object type flag: is visible in DataViewer (0x0001).
pub const TOBJ_TYPE_FLAG_VISIBLE_HP: i32 = 0x0001;

/// Local stand-in for TDF_Label.
pub type LabelEntryHp = String;

/// Local base: `TObj_Partition` — a named container of child objects.
#[derive(Debug)]
pub struct TObjPartitionBaseHp {
    pub label: LabelEntryHp,
    pub name: String,
    children: Vec<String>,
}

impl TObjPartitionBaseHp {
    pub fn new(label: &str) -> Self {
        TObjPartitionBaseHp {
            label: label.to_string(),
            name: String::new(),
            children: Vec::new(),
        }
    }

    /// TObj_Object::GetTypeFlags default: Visible.
    pub fn get_type_flags(&self) -> i32 {
        TOBJ_TYPE_FLAG_VISIBLE_HP
    }

    /// TObj_Partition::AddObject-style child registration.
    pub fn append_child(&mut self, name: &str) {
        self.children.push(name.to_string());
    }

    pub fn nb_children(&self) -> usize {
        self.children.len()
    }
}

/// Partition with predefined hidden flag.
#[derive(Debug)]
pub struct TObjHiddenPartition {
    /// The TObj_Partition base subobject.
    pub base: TObjPartitionBaseHp,
}

impl TObjHiddenPartition {
    /// The persistent type name registered by
    /// IMPLEMENT_TOBJOCAF_PERSISTENCE(TObj_HiddenPartition).
    pub const PERSISTENT_TYPE: &'static str = "TObj_HiddenPartition";

    /// Constructor by label.
    pub fn new(label: &str) -> Self {
        TObjHiddenPartition {
            base: TObjPartitionBaseHp::new(label),
        }
    }

    /// Returns all flags of the father except Visible.
    pub fn get_type_flags(&self) -> i32 {
        self.base.get_type_flags() & !TOBJ_TYPE_FLAG_VISIBLE_HP
    }

    /// Convenience: is this partition visible in the DataViewer?
    pub fn is_visible(&self) -> bool {
        (self.get_type_flags() & TOBJ_TYPE_FLAG_VISIBLE_HP) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_partition_is_visible() {
        let base = TObjPartitionBaseHp::new("0:3");
        assert_eq!(base.get_type_flags() & TOBJ_TYPE_FLAG_VISIBLE_HP, TOBJ_TYPE_FLAG_VISIBLE_HP);
    }

    #[test]
    fn hidden_partition_strips_visible_flag() {
        let hp = TObjHiddenPartition::new("0:3:1");
        assert_eq!(hp.get_type_flags() & TOBJ_TYPE_FLAG_VISIBLE_HP, 0);
        assert!(!hp.is_visible());
        // All non-Visible father flags are preserved (father has only
        // Visible by default, so the result is exactly 0).
        assert_eq!(hp.get_type_flags(), hp.base.get_type_flags() & !TOBJ_TYPE_FLAG_VISIBLE_HP);
    }

    #[test]
    fn partition_behavior_is_inherited() {
        let mut hp = TObjHiddenPartition::new("0:3:2");
        hp.base.append_child("bolt");
        hp.base.append_child("nut");
        assert_eq!(hp.base.nb_children(), 2);
        assert_eq!(hp.base.label, "0:3:2");
    }

    #[test]
    fn persistent_type_name() {
        assert_eq!(TObjHiddenPartition::PERSISTENT_TYPE, "TObj_HiddenPartition");
    }
}
