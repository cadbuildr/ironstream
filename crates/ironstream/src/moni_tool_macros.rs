// FILE: moni_tool_macros.rs
// occt: MoniTool_Macros

/// Macros for common operations are defined as functions/constants in Rust
/// Standard macros for monitoring tools

pub const MONI_TRACE_LEVEL_NONE: i32 = 0;
pub const MONI_TRACE_LEVEL_LOW: i32 = 1;
pub const MONI_TRACE_LEVEL_MEDIUM: i32 = 2;
pub const MONI_TRACE_LEVEL_HIGH: i32 = 3;

pub fn trace_level_name(level: i32) -> &'static str {
    match level {
        0 => "NONE",
        1 => "LOW",
        2 => "MEDIUM",
        3 => "HIGH",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_level() {
        assert_eq!(trace_level_name(0), "NONE");
        assert_eq!(trace_level_name(1), "LOW");
    }
}
