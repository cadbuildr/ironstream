// FILE: tk_math_pch.rs
// occt: TKMath_pch

//! Precompiled header for TKMath toolkit.
//! This is a marker module for precompiled header configuration.
//! In a Rust port, it serves as a namespace reference point.

/// Marker constant for PCH inclusion.
pub const PCH_INCLUDED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pch_included() {
        assert!(PCH_INCLUDED);
    }
}
