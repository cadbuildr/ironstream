// FILE: mesh_vs_mesh_ptr.rs
// occt: MeshVS_MeshPtr

//! Pointer type for mesh references.

pub type MeshPtr = Option<usize>;

impl MeshPtr {
    pub fn null() -> Self {
        None
    }

    pub fn from_raw(ptr: usize) -> Self {
        if ptr == 0 { None } else { Some(ptr) }
    }

    pub fn as_raw(&self) -> usize {
        self.unwrap_or(0)
    }

    pub fn is_null(&self) -> bool {
        self.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_ptr_null() {
        let ptr = MeshPtr::null();
        assert!(ptr.is_null());
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
}
