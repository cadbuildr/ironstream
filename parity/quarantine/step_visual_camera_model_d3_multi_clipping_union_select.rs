// FILE: step_visual_camera_model_d3_multi_clipping_union_select.rs
// occt: StepVisual_CameraModelD3MultiClippingUnionSelect

/// Represents a union for multi-clipping union selection
#[derive(Debug, Clone)]
pub enum StepVisual_CameraModelD3MultiClippingUnionSelect {
    Union(String),
    Intersection(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let sel = StepVisual_CameraModelD3MultiClippingUnionSelect::Union("u1".to_string());
        match sel {
            StepVisual_CameraModelD3MultiClippingUnionSelect::Union(ref u) => {
                assert_eq!(u, "u1")
            }
            _ => panic!("Expected Union"),
        }
    }
}
