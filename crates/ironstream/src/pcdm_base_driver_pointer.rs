// FILE: pcdm_base_driver_pointer.rs
// occt: PCDM_BaseDriverPointer

/// Represents a pointer to a storage base driver.
/// This is a forward declaration type with no public interface.
pub struct PCDMBaseDriverPointer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_type() {
        let _ptr = PCDMBaseDriverPointer;
    }
}
