// FILE: t_naming_tool.rs
// occt: TNaming_Tool

use std::collections::BTreeMap;

/// A tool to get information on the topology of a named shape attribute.
pub struct TNamingTool;

impl TNamingTool {
    pub fn current_shape(ns_id: i32, updated_labels: &BTreeMap<i32, bool>) -> Option<i32> {
        if ns_id < 0 {
            return None;
        }
        if !updated_labels.is_empty() && !updated_labels.contains_key(&ns_id) {
            return None;
        }
        Some(ns_id)
    }

    pub fn current_named_shape(ns_id: i32, updated_labels: &BTreeMap<i32, bool>) -> Option<i32> {
        Self::current_shape(ns_id, updated_labels)
    }

    pub fn named_shape(shape_id: i32, label_id: i32) -> Option<i32> {
        if shape_id < 0 || label_id < 0 {
            return None;
        }
        Some(shape_id)
    }

    pub fn get_shape(ns_id: i32) -> Option<i32> {
        if ns_id < 0 {
            return None;
        }
        Some(ns_id)
    }

    pub fn original_shape(ns_id: i32) -> Option<i32> {
        if ns_id < 0 {
            return None;
        }
        Some(ns_id)
    }

    pub fn generated_shape(source_shape: i32, generation_ns_id: i32) -> Option<i32> {
        if source_shape < 0 || generation_ns_id < 0 {
            return None;
        }
        Some(generation_ns_id)
    }

    pub fn collect(
        ns_id: i32,
        collected: &mut BTreeMap<i32, bool>,
        only_modif: bool,
    ) {
        if ns_id >= 0 {
            collected.insert(ns_id, only_modif);
        }
    }

    pub fn has_label(access_label: i32, shape_id: i32) -> bool {
        access_label >= 0 && shape_id >= 0
    }

    pub fn label(access_label: i32, shape_id: i32) -> Option<(i32, i32)> {
        if access_label >= 0 && shape_id >= 0 {
            Some((access_label, 0))
        } else {
            None
        }
    }

    pub fn initial_shape(
        shape_id: i32,
        access_label: i32,
        labels: &mut Vec<i32>,
    ) -> Option<i32> {
        if shape_id >= 0 && access_label >= 0 {
            labels.push(access_label);
            Some(shape_id)
        } else {
            None
        }
    }

    pub fn valid_until(access_label: i32, shape_id: i32) -> i32 {
        if access_label >= 0 && shape_id >= 0 {
            1
        } else {
            -1
        }
    }

    pub fn find_shape(
        valid_labels: &BTreeMap<i32, bool>,
        forbidden_labels: &BTreeMap<i32, bool>,
        arg_ns_id: i32,
        out_shape: &mut Option<i32>,
    ) {
        if arg_ns_id >= 0 && !forbidden_labels.contains_key(&arg_ns_id) {
            *out_shape = Some(arg_ns_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_shape_valid() {
        let mut updated = BTreeMap::new();
        updated.insert(1, true);
        assert_eq!(TNamingTool::current_shape(1, &updated), Some(1));
    }

    #[test]
    fn test_current_shape_invalid() {
        let updated = BTreeMap::new();
        assert_eq!(TNamingTool::current_shape(-1, &updated), None);
    }

    #[test]
    fn test_current_shape_not_in_updated() {
        let mut updated = BTreeMap::new();
        updated.insert(2, true);
        assert_eq!(TNamingTool::current_shape(1, &updated), None);
    }

    #[test]
    fn test_current_named_shape() {
        let mut updated = BTreeMap::new();
        updated.insert(1, true);
        assert_eq!(TNamingTool::current_named_shape(1, &updated), Some(1));
    }

    #[test]
    fn test_has_label() {
        assert!(TNamingTool::has_label(1, 2));
        assert!(!TNamingTool::has_label(-1, 2));
    }

    #[test]
    fn test_label() {
        assert_eq!(TNamingTool::label(1, 2), Some((1, 0)));
    }

    #[test]
    fn test_valid_until() {
        assert_eq!(TNamingTool::valid_until(1, 2), 1);
        assert_eq!(TNamingTool::valid_until(-1, 2), -1);
    }

    #[test]
    fn test_find_shape() {
        let mut valid = BTreeMap::new();
        valid.insert(1, true);
        let forbidden = BTreeMap::new();
        let mut out = None;
        
        TNamingTool::find_shape(&valid, &forbidden, 1, &mut out);
        assert_eq!(out, Some(1));
    }

    #[test]
    fn test_find_shape_forbidden() {
        let valid = BTreeMap::new();
        let mut forbidden = BTreeMap::new();
        forbidden.insert(1, true);
        let mut out = None;
        
        TNamingTool::find_shape(&valid, &forbidden, 1, &mut out);
        assert_eq!(out, None);
    }
}
