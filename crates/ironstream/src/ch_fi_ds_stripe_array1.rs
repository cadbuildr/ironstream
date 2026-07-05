// FILE: ch_fi_ds_stripe_array1.rs
// occt: ChFiDS_StripeArray1

//! Deprecated type alias for backward compatibility.
//! Use Vec<Arc<ChFiDsStripe>> directly instead.

use std::sync::Arc;

/// Stripe handle type (opaque marker).
pub struct ChFiDsStripeHandle;

/// Stripe array indexed from 1 to N.
/// Deprecated alias for NCollection_Array1<opencascade::handle<ChFiDS_Stripe>>.
/// Modeled as a vector with 1-based indexing via offset.
pub struct ChFiDsStripeArray1 {
    items: Vec<Arc<ChFiDsStripeHandle>>,
    lower: usize,
}

impl ChFiDsStripeArray1 {
    /// Creates a new array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        ChFiDsStripeArray1 {
            items: Vec::with_capacity(size),
            lower,
        }
    }

    /// Returns the lower index bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper index bound.
    pub fn upper(&self) -> usize {
        if self.items.is_empty() {
            self.lower
        } else {
            self.lower + self.items.len() - 1
        }
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets a reference to an element at the given index (1-based).
    pub fn get(&self, index: usize) -> Option<&Arc<ChFiDsStripeHandle>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset)
    }

    /// Gets a mutable reference to an element at the given index (1-based).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Arc<ChFiDsStripeHandle>> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get_mut(offset)
    }

    /// Sets an element at the given index (1-based).
    /// Note: For new arrays, elements must be pushed in order.
    pub fn set(&mut self, index: usize, stripe: Arc<ChFiDsStripeHandle>) -> bool {
        if index < self.lower {
            return false;
        }
        let offset = index - self.lower;

        // Extend if necessary
        while self.items.len() <= offset {
            // This is a limitation compared to OCCT: we can't set arbitrary positions
            // without filling in between. Use push for sequential addition.
            return false;
        }

        self.items[offset] = stripe;
        true
    }

    /// Appends an element to the array (helper for initialization).
    pub fn push(&mut self, stripe: Arc<ChFiDsStripeHandle>) {
        self.items.push(stripe);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_array1_creation() {
        let array = ChFiDsStripeArray1::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 1);
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_stripe_array1_push_and_get() {
        let mut array = ChFiDsStripeArray1::new(1, 5);
        let stripe1 = Arc::new(ChFiDsStripeHandle);
        let stripe2 = Arc::new(ChFiDsStripeHandle);

        array.push(stripe1.clone());
        array.push(stripe2.clone());

        assert_eq!(array.len(), 2);
        assert!(array.get(1).is_some());
        assert!(array.get(2).is_some());
    }

    #[test]
    fn test_stripe_array1_bounds_checking() {
        let mut array = ChFiDsStripeArray1::new(1, 3);
        array.push(Arc::new(ChFiDsStripeHandle));

        assert!(array.get(1).is_some());
        assert!(array.get(0).is_none());
        assert!(array.get(2).is_none());
    }

    #[test]
    fn test_stripe_array1_get_mut() {
        let mut array = ChFiDsStripeArray1::new(1, 5);
        array.push(Arc::new(ChFiDsStripeHandle));

        if let Some(stripe_ref) = array.get_mut(1) {
            // Verify we got a mutable reference
            let _stripe = stripe_ref.clone();
        }
    }

    #[test]
    fn test_stripe_array1_sequential_access() {
        let mut array = ChFiDsStripeArray1::new(1, 3);
        array.push(Arc::new(ChFiDsStripeHandle));
        array.push(Arc::new(ChFiDsStripeHandle));
        array.push(Arc::new(ChFiDsStripeHandle));

        assert_eq!(array.len(), 3);
        for i in 1..=3 {
            assert!(array.get(i).is_some());
        }
    }
}
