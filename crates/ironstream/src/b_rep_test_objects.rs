// FILE: b_rep_test_objects.rs
// occt: BRepTest_Objects

use std::sync::Mutex;

pub struct BrepTestObjects {
    objects: Mutex<Vec<usize>>,
}

impl BrepTestObjects {
    pub fn new() -> Self {
        BrepTestObjects {
            objects: Mutex::new(Vec::new()),
        }
    }

    pub fn add_object(&self, object_id: usize) {
        if let Ok(mut objs) = self.objects.lock() {
            objs.push(object_id);
        }
    }

    pub fn len(&self) -> usize {
        self.objects.lock().ok().map(|o| o.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.objects.lock().ok().map(|o| o.is_empty()).unwrap_or(true)
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.objects.lock().ok().and_then(|o| o.get(index).copied())
    }

    pub fn clear(&self) {
        if let Ok(mut objs) = self.objects.lock() {
            objs.clear();
        }
    }

    pub fn objects(&self) -> Vec<usize> {
        self.objects.lock().ok().map(|o| o.clone()).unwrap_or_default()
    }
}

impl Default for BrepTestObjects {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let objects = BrepTestObjects::new();
        assert!(objects.is_empty());
    }

    #[test]
    fn test_add_object() {
        let objects = BrepTestObjects::new();
        objects.add_object(1);
        assert_eq!(objects.len(), 1);
    }

    #[test]
    fn test_get_object() {
        let objects = BrepTestObjects::new();
        objects.add_object(42);
        assert_eq!(objects.get(0), Some(42));
    }

    #[test]
    fn test_clear() {
        let objects = BrepTestObjects::new();
        objects.add_object(1);
        objects.add_object(2);
        objects.clear();
        assert!(objects.is_empty());
    }
}
