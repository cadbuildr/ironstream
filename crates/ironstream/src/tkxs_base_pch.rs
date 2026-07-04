// FILE: tkxs_base_pch.rs
// occt: TKXSBase_pch

/// Precompiled header module for TKXSBase toolkit.
/// This module re-exports commonly used types from the data exchange framework.
/// In OCCT, this is a precompiled header file; here it serves as a namespace convenience.

/// Re-export common check and iteration types used across the data exchange toolkit.
pub use crate::interface_check::InterfaceCheck;

/// Marker that indicates this module has been included.
pub const PCH_INCLUDED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pch_marker() {
        assert!(PCH_INCLUDED);
    }
}
