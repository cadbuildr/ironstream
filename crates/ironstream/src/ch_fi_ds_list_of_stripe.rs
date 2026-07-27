// FILE: ch_fi_ds_list_of_stripe.rs
// occt: ChFiDS_ListOfStripe
// occt-ref: ChFiDS_ListIteratorOfListOfStripe

//! Deprecated type aliases for backward compatibility.
//! Use VecDeque<Arc<ChFiDsStripe>> directly instead.

use std::sync::Arc;
use std::collections::VecDeque;

/// Stripe handle type (opaque marker).
pub struct ChFiDsStripeHandle;

/// Deprecated list of Stripe handles.
/// Maps to NCollection_List<opencascade::handle<ChFiDS_Stripe>>.
pub type ChFiDsListOfStripe = VecDeque<Arc<ChFiDsStripeHandle>>;

/// Deprecated iterator over a list of Stripe handles.
/// Maps to NCollection_List<...>::Iterator.
pub struct ChFiDsListIteratorOfListOfStripe<'a> {
    items: &'a VecDeque<Arc<ChFiDsStripeHandle>>,
    index: usize,
}

impl<'a> ChFiDsListIteratorOfListOfStripe<'a> {
    /// Creates a new iterator over the list.
    pub fn new(list: &'a ChFiDsListOfStripe) -> Self {
        ChFiDsListIteratorOfListOfStripe {
            items: list,
            index: 0,
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Returns a reference to the current element.
    pub fn value(&self) -> Option<&Arc<ChFiDsStripeHandle>> {
        self.items.get(self.index)
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl<'a> Iterator for ChFiDsListIteratorOfListOfStripe<'a> {
    type Item = &'a Arc<ChFiDsStripeHandle>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.items.get(self.index);
        if current.is_some() {
            self.index += 1;
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_of_stripe_creation() {
        let list: ChFiDsListOfStripe = VecDeque::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_of_stripe_push() {
        let mut list: ChFiDsListOfStripe = VecDeque::new();
        let stripe = Arc::new(ChFiDsStripeHandle);
        list.push_back(stripe.clone());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_iterator_of_stripe() {
        let mut list: ChFiDsListOfStripe = VecDeque::new();
        list.push_back(Arc::new(ChFiDsStripeHandle));
        list.push_back(Arc::new(ChFiDsStripeHandle));
        list.push_back(Arc::new(ChFiDsStripeHandle));

        let mut iter = ChFiDsListIteratorOfListOfStripe::new(&list);
        assert!(iter.more());
        assert!(iter.value().is_some());

        iter.next();
        assert!(iter.more());

        iter.next();
        assert!(iter.more());

        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_list_iterator_of_stripe_as_rust_iterator() {
        let mut list: ChFiDsListOfStripe = VecDeque::new();
        list.push_back(Arc::new(ChFiDsStripeHandle));
        list.push_back(Arc::new(ChFiDsStripeHandle));

        let iter = ChFiDsListIteratorOfListOfStripe::new(&list);
        let count = iter.count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_list_operations() {
        let mut list: ChFiDsListOfStripe = VecDeque::new();
        let stripe1 = Arc::new(ChFiDsStripeHandle);
        let stripe2 = Arc::new(ChFiDsStripeHandle);

        list.push_back(stripe1);
        list.push_back(stripe2);

        assert_eq!(list.len(), 2);
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        list.clear();
        assert!(list.is_empty());
    }
}
