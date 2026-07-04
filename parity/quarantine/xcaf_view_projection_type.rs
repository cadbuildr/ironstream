// FILE: xcaf_view_projection_type.rs
// occt: XCAFView_ProjectionType

/// Enum representing XCAFView_ProjectionType from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum XCAFView_ProjectionType {
    NoCamera,
    Parallel,
    Central,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcaf_view_projection_type_variants() {
        let _ = XCAFView_ProjectionType::NoCamera;
        let _ = XCAFView_ProjectionType::Parallel;
        let _ = XCAFView_ProjectionType::Central;
    }
}
