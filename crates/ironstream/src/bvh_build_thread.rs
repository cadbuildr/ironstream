// FILE: bvh_build_thread.rs
// occt: BVH_BuildThread

/// Parallel BVH tree builder thread.
pub struct BvhBuildThread {
    thread_id: usize,
}

impl BvhBuildThread {
    pub fn new(id: usize) -> Self {
        Self { thread_id: id }
    }

    pub fn thread_id(&self) -> usize {
        self.thread_id
    }
}
