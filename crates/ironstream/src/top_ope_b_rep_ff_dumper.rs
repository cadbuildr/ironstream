// FILE: top_ope_b_rep_ff_dumper.rs
// occt: TopOpeBRep_FFDumper

/// FFDumper stub implementation.
pub struct FFDumper;

impl FFDumper {
    /// Create new instance.
    pub fn new() -> Self {
        FFDumper
    }
}

impl Default for FFDumper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = FFDumper::new();
    }

    #[test]
    fn test_default() {
        let _ = FFDumper::default();
    }
}
