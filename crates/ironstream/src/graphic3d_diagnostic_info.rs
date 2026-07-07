// FILE: graphic3d_diagnostic_info.rs
// occt: Graphic3d_DiagnosticInfo

//! Diagnostic info categories bit flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DiagnosticInfo {
    /// device / vendor / version information
    Device = 0x001,
    /// framebuffer information
    FrameBuffer = 0x002,
    /// hardware limits
    Limits = 0x004,
    /// memory counters
    Memory = 0x008,
    /// native platform API information (e.g. WGL / GLX / EGL)
    NativePlatform = 0x010,
    /// vendor extension list (usually very long)
    Extensions = 0x020,
}

/// Minimal information (Device | FrameBuffer | Limits)
pub const DIAGNOSTIC_INFO_SHORT: i32 = (DiagnosticInfo::Device as i32)
    | (DiagnosticInfo::FrameBuffer as i32)
    | (DiagnosticInfo::Limits as i32);

/// Basic information, without extension list (Short | NativePlatform | Memory)
pub const DIAGNOSTIC_INFO_BASIC: i32 =
    DIAGNOSTIC_INFO_SHORT | (DiagnosticInfo::NativePlatform as i32) | (DiagnosticInfo::Memory as i32);

/// Complete information, including extension list (Basic | Extensions)
pub const DIAGNOSTIC_INFO_COMPLETE: i32 =
    DIAGNOSTIC_INFO_BASIC | (DiagnosticInfo::Extensions as i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_info_flags() {
        assert_eq!(DiagnosticInfo::Device as i32, 0x001);
        assert_eq!(DiagnosticInfo::FrameBuffer as i32, 0x002);
        assert_eq!(DiagnosticInfo::Limits as i32, 0x004);
        assert_eq!(DiagnosticInfo::Memory as i32, 0x008);
        assert_eq!(DiagnosticInfo::NativePlatform as i32, 0x010);
        assert_eq!(DiagnosticInfo::Extensions as i32, 0x020);
    }

    #[test]
    fn test_diagnostic_info_groups() {
        // SHORT = Device | FrameBuffer | Limits
        assert_eq!(DIAGNOSTIC_INFO_SHORT, 0x001 | 0x002 | 0x004);
        assert_eq!(DIAGNOSTIC_INFO_SHORT, 0x007);

        // BASIC = SHORT | NativePlatform | Memory
        assert_eq!(DIAGNOSTIC_INFO_BASIC, 0x007 | 0x010 | 0x008);
        assert_eq!(DIAGNOSTIC_INFO_BASIC, 0x01F);

        // COMPLETE = BASIC | Extensions
        assert_eq!(DIAGNOSTIC_INFO_COMPLETE, 0x01F | 0x020);
        assert_eq!(DIAGNOSTIC_INFO_COMPLETE, 0x03F);
    }

    #[test]
    fn test_diagnostic_info_bitwise_operations() {
        // Test that flags can be combined
        let combined = (DiagnosticInfo::Device as i32) | (DiagnosticInfo::Memory as i32);
        assert_eq!(combined, 0x001 | 0x008);
        assert_eq!(combined, 0x009);

        // Test membership check
        assert_eq!(combined & (DiagnosticInfo::Device as i32), 0x001);
        assert_eq!(combined & (DiagnosticInfo::Memory as i32), 0x008);
        assert_eq!(combined & (DiagnosticInfo::Extensions as i32), 0);
    }
}
