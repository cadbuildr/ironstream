// FILE: storage_bucket_of_persistent.rs
// occt: Storage_BucketOfPersistent

//! Bucket storage for persistent objects.

use std::sync::Arc;

/// Bucket for storing persistent objects.
#[derive(Debug, Clone)]
pub struct StorageBucketOfPersistent {
    items: Vec<Arc<dyn std::any::Any>>,
    bucket_size: usize,
}

impl StorageBucketOfPersistent {
    /// Create a new bucket for persistent objects.
    pub fn new(bucket_size: usize, _bucket_number: usize) -> Self {
        StorageBucketOfPersistent {
            items: Vec::new(),
            bucket_size,
        }
    }

    /// Default constructor.
    pub fn default_new() -> Self {
        StorageBucketOfPersistent::new(300000, 100)
    }

    /// Get the length of the bucket.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Append an item to the bucket.
    pub fn append(&mut self, item: Arc<dyn std::any::Any>) {
        if self.items.len() < self.bucket_size {
            self.items.push(item);
        }
    }

    /// Get the item at the given index.
    pub fn value(&self, index: usize) -> Option<Arc<dyn std::any::Any>> {
        if index < self.items.len() {
            Some(Arc::clone(&self.items[index]))
        } else {
            None
        }
    }

    /// Clear the bucket.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for StorageBucketOfPersistent {
    fn default() -> Self {
        Self::default_new()
    }
}

/// Iterator for storage bucket.
pub struct StorageBucketIterator {
    items: Vec<Arc<dyn std::any::Any>>,
    current_index: usize,
}

impl StorageBucketIterator {
    /// Create a new iterator for the bucket.
    pub fn new(bucket: &StorageBucketOfPersistent) -> Self {
        StorageBucketIterator {
            items: bucket.items.clone(),
            current_index: 0,
        }
    }

    /// Reset the iterator.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get the current value.
    pub fn value(&self) -> Option<Arc<dyn std::any::Any>> {
        if self.current_index < self.items.len() {
            Some(Arc::clone(&self.items[self.current_index]))
        } else {
            None
        }
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.current_index < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        self.current_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_creation() {
        let bucket = StorageBucketOfPersistent::new(1000, 10);
        assert!(bucket.is_empty());
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn test_bucket_append() {
        let mut bucket = StorageBucketOfPersistent::new(1000, 10);
        let item = Arc::new(42i32);
        bucket.append(item);
        assert_eq!(bucket.len(), 1);
    }

    #[test]
    fn test_bucket_value() {
        let mut bucket = StorageBucketOfPersistent::new(1000, 10);
        let item = Arc::new(42i32);
        bucket.append(item);
        assert!(bucket.value(0).is_some());
        assert!(bucket.value(1).is_none());
    }

    #[test]
    fn test_bucket_clear() {
        let mut bucket = StorageBucketOfPersistent::new(1000, 10);
        let item = Arc::new(42i32);
        bucket.append(item);
        bucket.clear();
        assert!(bucket.is_empty());
    }

    #[test]
    fn test_bucket_iterator() {
        let mut bucket = StorageBucketOfPersistent::new(1000, 10);
        bucket.append(Arc::new(1i32));
        bucket.append(Arc::new(2i32));

        let mut iter = StorageBucketIterator::new(&bucket);
        assert!(iter.more());
        assert!(iter.value().is_some());
        iter.next();
        assert!(iter.more());
        iter.next();
        assert!(!iter.more());
    }
}
