// FILE: t_data_std.rs
// occt: TDataStd

/// Namespace module for standard data attributes.
pub struct TDataStd;

impl TDataStd {
    /// Returns the module name.
    pub fn name() -> &'static str {
        "TDataStd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_data_std_name() {
        assert_eq!(TDataStd::name(), "TDataStd");
    }
}
