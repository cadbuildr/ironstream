// FILE: t_naming_name.rs
// occt: TNaming_Name

use std::collections::BTreeMap;

/// Represents a shape's topological type/enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
    Shape,
}

/// Type of naming transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameType {
    Unknown,
    Identity,
    Modif,
    Generated,
    FilterByNeighbours,
    Intersection,
    Union,
    ConstShape,
    OrientationModified,
    Orientation,
    WireIN,
    Restrict,
    Substitution,
    TransformedShape,
}

/// Represents a topological orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Stores the arguments of naming transformations.
#[derive(Debug, Clone)]
pub struct TNamingName {
    args: Vec<i32>,
    stop_shape: Option<i32>,
    name_type: NameType,
    shape_type: ShapeType,
    shape_data: Vec<u8>,
    index: i32,
    context_label_id: i32,
    orientation: Orientation,
}

impl TNamingName {
    pub fn new() -> Self {
        TNamingName {
            args: Vec::new(),
            stop_shape: None,
            name_type: NameType::Unknown,
            shape_type: ShapeType::Shape,
            shape_data: Vec::new(),
            index: -1,
            context_label_id: -1,
            orientation: Orientation::Forward,
        }
    }

    pub fn set_type(&mut self, ty: NameType) {
        self.name_type = ty;
    }

    pub fn get_type(&self) -> NameType {
        self.name_type
    }

    pub fn set_shape_type(&mut self, st: ShapeType) {
        self.shape_type = st;
    }

    pub fn get_shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn set_shape(&mut self, shape_data: Vec<u8>) {
        self.shape_data = shape_data;
    }

    pub fn get_shape(&self) -> &[u8] {
        &self.shape_data
    }

    pub fn append_argument(&mut self, arg: i32) {
        self.args.push(arg);
    }

    pub fn set_stop_named_shape(&mut self, stop: i32) {
        self.stop_shape = Some(stop);
    }

    pub fn get_stop_named_shape(&self) -> Option<i32> {
        self.stop_shape
    }

    pub fn set_index(&mut self, idx: i32) {
        self.index = idx;
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn set_context_label(&mut self, label_id: i32) {
        self.context_label_id = label_id;
    }

    pub fn get_context_label(&self) -> i32 {
        self.context_label_id
    }

    pub fn get_arguments(&self) -> &[i32] {
        &self.args
    }

    pub fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn set_orientation(&mut self, orient: Orientation) {
        self.orientation = orient;
    }

    pub fn solve(&self, valid_labels: &BTreeMap<i32, bool>) -> bool {
        if self.args.is_empty() {
            return false;
        }

        for &arg in &self.args {
            if arg < 0 {
                return false;
            }
            if !valid_labels.is_empty() && !valid_labels.contains_key(&arg) {
                return false;
            }
        }

        true
    }

    pub fn paste(&self, relocation_map: &BTreeMap<i32, i32>) -> TNamingName {
        let mut into = TNamingName::new();
        into.name_type = self.name_type;
        into.shape_type = self.shape_type;
        into.shape_data = self.shape_data.clone();
        into.index = self.index;

        for &arg in &self.args {
            let relocated = relocation_map.get(&arg).copied().unwrap_or(arg);
            into.args.push(relocated);
        }

        if let Some(stop) = self.stop_shape {
            into.stop_shape = Some(relocation_map.get(&stop).copied().unwrap_or(stop));
        }

        if self.context_label_id >= 0 {
            into.context_label_id = relocation_map
                .get(&self.context_label_id)
                .copied()
                .unwrap_or(self.context_label_id);
        }

        into.orientation = self.orientation;
        into
    }
}

impl Default for TNamingName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default() {
        let name = TNamingName::new();
        assert_eq!(name.get_type(), NameType::Unknown);
        assert_eq!(name.get_shape_type(), ShapeType::Shape);
        assert_eq!(name.get_index(), -1);
        assert_eq!(name.get_arguments().len(), 0);
        assert_eq!(name.get_stop_named_shape(), None);
    }

    #[test]
    fn test_set_get_type() {
        let mut name = TNamingName::new();
        name.set_type(NameType::Identity);
        assert_eq!(name.get_type(), NameType::Identity);
    }

    #[test]
    fn test_append_argument() {
        let mut name = TNamingName::new();
        name.append_argument(1);
        name.append_argument(2);
        assert_eq!(name.get_arguments(), &[1, 2]);
    }

    #[test]
    fn test_solve_empty_args() {
        let name = TNamingName::new();
        let valid_labels = BTreeMap::new();
        assert!(!name.solve(&valid_labels));
    }

    #[test]
    fn test_solve_with_valid_args() {
        let mut name = TNamingName::new();
        name.append_argument(1);

        let mut valid_labels = BTreeMap::new();
        valid_labels.insert(1, true);

        assert!(name.solve(&valid_labels));
    }

    #[test]
    fn test_paste_relocates() {
        let mut name = TNamingName::new();
        name.append_argument(1);

        let mut relocation_map = BTreeMap::new();
        relocation_map.insert(1, 100);

        let pasted = name.paste(&relocation_map);
        assert_eq!(pasted.get_arguments(), &[100]);
    }

    #[test]
    fn test_buc60925_solve_with_empty_args() {
        let name = TNamingName::new();
        let valid_labels = BTreeMap::new();
        assert!(!name.solve(&valid_labels));
    }
}
