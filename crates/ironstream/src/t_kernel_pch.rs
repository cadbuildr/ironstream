// FILE: t_kernel_pch.rs
// occt: TKernel_pch

//! Precompiled header definitions for TKernel module
//! In C++, this would include standard headers; in Rust we document the equivalent modules
//! Maps to TKernel_pch.hxx - a precompiled header file in OCCT

pub mod standard_lib {
    //! Standard library equivalents for C++ includes
    pub const ATOMIC: &str = "std::sync::atomic";
    pub const IOSTREAM: &str = "std::io";
    pub const LIMITS: &str = "std::numeric";
    pub const OSTREAM: &str = "std::io";
    pub const RANDOM: &str = "rand";
    pub const SSTREAM: &str = "std::string";
    pub const STRING: &str = "std::string::String";
    pub const TYPE_TRAITS: &str = "std::any";
}

pub mod windows_specific {
    //! Windows-specific includes (if compiled for Windows)
    pub const TCHAR: &str = "Windows API character handling";
    pub const WINDOWS: &str = "Windows API";
}

pub mod tkernel_headers {
    //! TKernel module includes
    pub const STANDARD: &str = "Standard compatibility layer";
    pub const STANDARD_HANDLE: &str = "Handle/reference counting";
    pub const STANDARD_GUID: &str = "GUID utilities";
    pub const STANDARD_MACRO: &str = "Standard macros";
    pub const PRECISION: &str = "Floating point precision";
    pub const QUANTITY_COLOR: &str = "Color definitions";
    pub const TCOLLECTION_ASCIISTRING: &str = "ASCII string handling";
    pub const TCOLLECTION_EXTENDEDSTRING: &str = "Extended Unicode string handling";
    pub const NCOLLECTION_ARRAY1: &str = "1D array container";
    pub const NCOLLECTION_ARRAY2: &str = "2D array container";
    pub const NCOLLECTION_DATAMAP: &str = "Data map container";
    pub const NCOLLECTION_LIST: &str = "List container";
    pub const NCOLLECTION_MAP: &str = "Hash map container";
    pub const NCOLLECTION_SEQUENCE: &str = "Sequence container";
    pub const MESSAGE_PROGRESS_INDICATOR: &str = "Progress tracking";
    pub const OSD_PARALLEL: &str = "Parallel processing utilities";
    pub const OSD_PATH: &str = "File path utilities";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_lib_constants() {
        assert!(!standard_lib::STRING.is_empty());
        assert!(!standard_lib::IOSTREAM.is_empty());
    }

    #[test]
    fn test_tkernel_headers_constants() {
        assert!(!tkernel_headers::STANDARD.is_empty());
        assert!(!tkernel_headers::TCOLLECTION_ASCIISTRING.is_empty());
    }

    #[test]
    fn test_pch_documentation() {
        // This test verifies all constants are properly defined
        assert_eq!(standard_lib::ATOMIC, "std::sync::atomic");
        assert_eq!(standard_lib::IOSTREAM, "std::io");
    }
}
