// FILE: interface_vector_of_file_parameter.rs
// occt: Interface_VectorOfFileParameter

/// Deprecated alias for NCollection_DynamicArray<Interface_FileParameter>.
/// Maintains backward compatibility. Use Vec directly in new code.
pub struct InterfaceVectorOfFileParameter {
    items: Vec<u32>, // Placeholder for Interface_FileParameter (opaque type)
}

impl InterfaceVectorOfFileParameter {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn insert_before(&mut self, index: usize, item: u32) {
        if index <= self.items.len() {
            self.items.insert(index, item);
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<u32> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn value_at(&self, index: usize) -> Option<u32> {
        if index < self.items.len() {
            Some(self.items[index])
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for InterfaceVectorOfFileParameter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_append() {
        let mut vec = InterfaceVectorOfFileParameter::new();
        assert!(vec.is_empty());

        vec.append(10);
        vec.append(20);
        vec.append(30);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.value_at(0), Some(10));
        assert_eq!(vec.value_at(1), Some(20));
        assert_eq!(vec.value_at(2), Some(30));
    }

    #[test]
    fn test_vector_insert_before() {
        let mut vec = InterfaceVectorOfFileParameter::new();
        vec.append(10);
        vec.append(30);
        vec.insert_before(1, 20);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.value_at(0), Some(10));
        assert_eq!(vec.value_at(1), Some(20));
        assert_eq!(vec.value_at(2), Some(30));
    }

    #[test]
    fn test_vector_remove_at() {
        let mut vec = InterfaceVectorOfFileParameter::new();
        vec.append(10);
        vec.append(20);
        vec.append(30);

        assert_eq!(vec.remove_at(1), Some(20));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.value_at(1), Some(30));
    }

    #[test]
    fn test_vector_clear() {
        let mut vec = InterfaceVectorOfFileParameter::new();
        vec.append(1);
        vec.append(2);
        vec.append(3);
        assert_eq!(vec.len(), 3);

        vec.clear();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_default() {
        let vec = InterfaceVectorOfFileParameter::default();
        assert!(vec.is_empty());
    }
}
