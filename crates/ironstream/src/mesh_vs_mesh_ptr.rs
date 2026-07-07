// FILE: mesh_vs_mesh_ptr.rs
// occt: MeshVS_MeshPtr

//! Pointer type for mesh references (typedef MeshVS_Mesh* MeshVS_MeshPtr).
//! Modelled as a newtype over an optional raw address value.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MeshPtr(Option<usize>);

impl MeshPtr {
    /// Null pointer.
    pub fn null() -> Self {
        MeshPtr(None)
    }

    /// Build from a raw address value; 0 maps to null.
    pub fn from_raw(ptr: usize) -> Self {
        if ptr == 0 {
            MeshPtr(None)
        } else {
            MeshPtr(Some(ptr))
        }
    }

    /// Raw address value; null maps to 0.
    pub fn as_raw(&self) -> usize {
        self.0.unwrap_or(0)
    }

    /// True if the pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_ptr_null() {
        let ptr = MeshPtr::null();
        assert!(ptr.is_null());
        assert_eq!(ptr.as_raw(), 0);
    }

    #[test]
    fn mesh_ptr_from_raw() {
        let ptr = MeshPtr::from_raw(0x5678);
        assert!(!ptr.is_null());
        assert_eq!(ptr.as_raw(), 0x5678);
    }

    #[test]
    fn mesh_ptr_zero() {
        let ptr = MeshPtr::from_raw(0);
        assert!(ptr.is_null());
    }

    #[test]
    fn mesh_ptr_default_is_null() {
        assert!(MeshPtr::default().is_null());
    }
}
