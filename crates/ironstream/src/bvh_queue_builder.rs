// FILE: bvh_queue_builder.rs
// occt: BVH_QueueBuilder

/// Queue-based BVH builder.
pub struct BvhQueueBuilder;

impl BvhQueueBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BvhQueueBuilder {
    fn default() -> Self {
        Self::new()
    }
}
