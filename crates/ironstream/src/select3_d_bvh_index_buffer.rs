// FILE: select3_d_bvh_index_buffer.rs
// occt: Select3D_BVHIndexBuffer

/// Stub for Select3D_BVHIndexBuffer from OCCT.
#[derive(Clone, Debug)]
pub struct Select3D_BVHIndexBuffer {}

impl Select3D_BVHIndexBuffer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Select3D_BVHIndexBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select3_d_bvh_index_buffer_creation() {
        let _obj = Select3D_BVHIndexBuffer::new();
        let _def = Select3D_BVHIndexBuffer::default();
    }
}
