// FILE: i_mesh_data_types.rs
// occt: IMeshData_Types

/// Type definitions and constants for IMeshData.
/// This module provides type aliases and memory management constants
/// for mesh data structures.

/// Default size for memory block allocated by IncAllocator.
#[cfg(target_pointer_width = "64")]
pub const MEMORY_BLOCK_SIZE_HUGE: usize = 1024 * 1024; // 1 MiB on WIN64

#[cfg(not(target_pointer_width = "64"))]
pub const MEMORY_BLOCK_SIZE_HUGE: usize = 512 * 1024; // 512 KiB elsewhere

/// Edge pointer type alias.
pub type IEdgePtr = usize;

/// Face pointer type alias.
pub type IFacePtr = usize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_block_size() {
        assert!(MEMORY_BLOCK_SIZE_HUGE > 0);
    }

    #[test]
    fn test_type_aliases() {
        let _edge: IEdgePtr = 0;
        let _face: IFacePtr = 1;
    }
}
