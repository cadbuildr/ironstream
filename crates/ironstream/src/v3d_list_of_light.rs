// FILE: v3d_list_of_light.rs
// occt: V3d_ListOfLight

use std::collections::LinkedList;

/// Placeholder for Graphic3d_CLight handle
#[derive(Clone, Debug)]
pub struct Graphic3dCLightHandle {
    // In real implementation, would contain reference-counted pointer to Graphic3d_CLight
}

/// Deprecated typedef: NCollection_List<opencascade::handle<Graphic3d_CLight>>
///
/// A list of light objects (Graphic3d_CLight handles).
/// Supports forward iteration over light sources.
#[derive(Clone, Debug)]
pub struct V3dListOfLight {
    items: LinkedList<Graphic3dCLightHandle>,
}

/// Iterator for V3dListOfLight
pub struct V3dListOfLightIterator<'a> {
    inner: std::collections::linked_list::Iter<'a, Graphic3dCLightHandle>,
}

impl V3dListOfLight {
    /// Create an empty list.
    pub fn new() -> Self {
        V3dListOfLight {
            items: LinkedList::new(),
        }
    }

    /// Append a light to the list.
    pub fn append(&mut self, light: Graphic3dCLightHandle) {
        self.items.push_back(light);
    }

    /// Prepend a light to the list.
    pub fn prepend(&mut self, light: Graphic3dCLightHandle) {
        self.items.push_front(light);
    }

    /// Get the size of the list.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get a forward iterator.
    pub fn iter(&self) -> V3dListOfLightIterator {
        V3dListOfLightIterator {
            inner: self.items.iter(),
        }
    }

    /// Get a mutable reference to the front element.
    pub fn first(&self) -> Option<&Graphic3dCLightHandle> {
        self.items.front()
    }

    /// Get a mutable reference to the back element.
    pub fn last(&self) -> Option<&Graphic3dCLightHandle> {
        self.items.back()
    }
}

impl<'a> Iterator for V3dListOfLightIterator<'a> {
    type Item = &'a Graphic3dCLightHandle;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl Default for V3dListOfLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        let list = V3dListOfLight::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_append_light() {
        let mut list = V3dListOfLight::new();
        list.append(Graphic3dCLightHandle {});
        assert_eq!(list.size(), 1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_prepend_light() {
        let mut list = V3dListOfLight::new();
        list.append(Graphic3dCLightHandle {});
        list.prepend(Graphic3dCLightHandle {});
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_first_last() {
        let mut list = V3dListOfLight::new();
        let light = Graphic3dCLightHandle {};
        list.append(light.clone());
        assert!(list.first().is_some());
        assert!(list.last().is_some());
    }

    #[test]
    fn test_clear_list() {
        let mut list = V3dListOfLight::new();
        list.append(Graphic3dCLightHandle {});
        list.append(Graphic3dCLightHandle {});
        assert_eq!(list.size(), 2);
        list.clear();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut list = V3dListOfLight::new();
        list.append(Graphic3dCLightHandle {});
        list.append(Graphic3dCLightHandle {});
        list.append(Graphic3dCLightHandle {});

        let count = list.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_multiple_iterations() {
        let mut list = V3dListOfLight::new();
        for _ in 0..5 {
            list.append(Graphic3dCLightHandle {});
        }

        // First iteration
        let count1 = list.iter().count();
        // Second iteration (should work again)
        let count2 = list.iter().count();

        assert_eq!(count1, 5);
        assert_eq!(count2, 5);
    }

    #[test]
    fn test_empty_iteration() {
        let list = V3dListOfLight::new();
        let count = list.iter().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_default_trait() {
        let list = V3dListOfLight::default();
        assert!(list.is_empty());
    }

    #[test]
    fn test_clone() {
        let mut list1 = V3dListOfLight::new();
        list1.append(Graphic3dCLightHandle {});
        let list2 = list1.clone();
        assert_eq!(list1.size(), list2.size());
    }
}
