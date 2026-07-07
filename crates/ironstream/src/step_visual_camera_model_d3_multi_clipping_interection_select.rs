// FILE: step_visual_camera_model_d3_multi_clipping_interection_select.rs
// occt: StepVisual_CameraModelD3MultiClippingInterectionSelect

/// Represents a union for multi-clipping intersection selection
#[derive(Debug, Clone)]
pub enum StepVisual_CameraModelD3MultiClippingInterectionSelect {
    Intersection(String),
    Union(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let sel = StepVisual_CameraModelD3MultiClippingInterectionSelect::Intersection("int1".to_string());
        match sel {
            StepVisual_CameraModelD3MultiClippingInterectionSelect::Intersection(ref i) => {
                assert_eq!(i, "int1")
            }
            _ => panic!("Expected Intersection"),
        }
    }
}
