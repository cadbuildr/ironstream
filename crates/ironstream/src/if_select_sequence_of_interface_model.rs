// FILE: if_select_sequence_of_interface_model.rs
// occt: IFSelect_SequenceOfInterfaceModel

/// Represents an interface model.
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceInterfaceModel {
    pub id: usize,
}

/// Sequence of Interface_InterfaceModel objects.
/// In OCCT, this was NCollection_Sequence<opencascade::handle<Interface_InterfaceModel>>.
/// This Rust newtype wraps a Vec for faithful behavior.
pub struct IFSelectSequenceOfInterfaceModel {
    items: Vec<InterfaceInterfaceModel>,
}

impl IFSelectSequenceOfInterfaceModel {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        IFSelectSequenceOfInterfaceModel { items: Vec::new() }
    }

    /// Returns the length of the sequence.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Appends an element to the end of the sequence.
    pub fn append(&mut self, value: InterfaceInterfaceModel) {
        self.items.push(value);
    }

    /// Returns a reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value(&self, index: usize) -> Option<&InterfaceInterfaceModel> {
        if index > 0 && index <= self.items.len() {
            self.items.get(index - 1)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index (1-indexed for OCCT compatibility).
    pub fn value_mut(&mut self, index: usize) -> Option<&mut InterfaceInterfaceModel> {
        if index > 0 && index <= self.items.len() {
            self.items.get_mut(index - 1)
        } else {
            None
        }
    }

    /// Clears all elements from the sequence.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<InterfaceInterfaceModel> {
        self.items.iter()
    }
}

impl Default for IFSelectSequenceOfInterfaceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sequence() {
        let seq = IFSelectSequenceOfInterfaceModel::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_and_len() {
        let mut seq = IFSelectSequenceOfInterfaceModel::new();
        let model = InterfaceInterfaceModel { id: 1 };
        seq.append(model);
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_value_1indexed() {
        let mut seq = IFSelectSequenceOfInterfaceModel::new();
        let model = InterfaceInterfaceModel { id: 7 };
        seq.append(model.clone());

        // 1-indexed access
        assert_eq!(seq.value(1).unwrap().id, 7);
        // 0-indexed should return None
        assert_eq!(seq.value(0), None);
        // Out of bounds
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_value_mut() {
        let mut seq = IFSelectSequenceOfInterfaceModel::new();
        let model = InterfaceInterfaceModel { id: 2 };
        seq.append(model);

        if let Some(val) = seq.value_mut(1) {
            val.id = 25;
        }

        let retrieved = seq.value(1).unwrap();
        assert_eq!(retrieved.id, 25);
    }

    #[test]
    fn test_clear() {
        let mut seq = IFSelectSequenceOfInterfaceModel::new();
        seq.append(InterfaceInterfaceModel { id: 1 });
        seq.append(InterfaceInterfaceModel { id: 2 });

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert_eq!(seq.len(), 0);
        assert!(seq.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut seq = IFSelectSequenceOfInterfaceModel::new();
        let model1 = InterfaceInterfaceModel { id: 1 };
        let model2 = InterfaceInterfaceModel { id: 2 };
        seq.append(model1.clone());
        seq.append(model2.clone());

        let ids: Vec<usize> = seq.iter().map(|m| m.id).collect();
        assert_eq!(ids, vec![1, 2]);
    }
}
