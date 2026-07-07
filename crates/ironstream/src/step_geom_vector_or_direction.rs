// FILE: step_geom_vector_or_direction.rs
// occt: StepGeom_VectorOrDirection

#[derive(Clone, Debug)]
pub enum VectorOrDirectionType {
    Vector,
    Direction,
}

pub struct VectorOrDirection {
    case_num: i32,
    value: Option<Box<dyn std::any::Any>>,
}

impl VectorOrDirection {
    pub fn new() -> Self {
        VectorOrDirection {
            case_num: 0,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        self.case_num
    }

    pub fn set_case(&mut self, case: i32, value: Option<Box<dyn std::any::Any>>) {
        self.case_num = case;
        self.value = value;
    }

    pub fn vector(&self) -> Option<&Box<dyn std::any::Any>> {
        if self.case_num == 1 {
            self.value.as_ref()
        } else {
            None
        }
    }

    pub fn direction(&self) -> Option<&Box<dyn std::any::Any>> {
        if self.case_num == 2 {
            self.value.as_ref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_or_direction_creation() {
        let vod = VectorOrDirection::new();
        assert_eq!(vod.case_num(), 0);
        assert!(vod.vector().is_none());
        assert!(vod.direction().is_none());
    }

    #[test]
    fn test_vector_or_direction_set_case() {
        let mut vod = VectorOrDirection::new();
        vod.set_case(1, None);
        assert_eq!(vod.case_num(), 1);

        let mut vod2 = VectorOrDirection::new();
        vod2.set_case(2, None);
        assert_eq!(vod2.case_num(), 2);
    }

    #[test]
    fn test_vector_or_direction_accessors() {
        let mut vod = VectorOrDirection::new();
        vod.set_case(1, None);
        assert!(vod.vector().is_none());
        assert!(vod.direction().is_none());

        let mut vod2 = VectorOrDirection::new();
        vod2.set_case(2, None);
        assert!(vod2.vector().is_none());
        assert!(vod2.direction().is_none());
    }
}
