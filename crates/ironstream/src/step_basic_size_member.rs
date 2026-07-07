// FILE: step_basic_size_member.rs
// occt: StepBasic_SizeMember

#[derive(Clone, Debug)]
pub enum StepBasicSizeMember {
    LengthMeasure(f64),
    PlaneAngleMeasure(f64),
}

impl StepBasicSizeMember {
    pub fn case_num(&self) -> i32 {
        match self {
            Self::LengthMeasure(_) => 1,
            Self::PlaneAngleMeasure(_) => 2,
        }
    }

    pub fn length_measure(&self) -> Option<f64> {
        match self { Self::LengthMeasure(v) => Some(*v), _ => None }
    }

    pub fn plane_angle_measure(&self) -> Option<f64> {
        match self { Self::PlaneAngleMeasure(v) => Some(*v), _ => None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        let l = StepBasicSizeMember::LengthMeasure(10.5);
        assert_eq!(l.case_num(), 1);
        assert_eq!(l.length_measure(), Some(10.5));

        let a = StepBasicSizeMember::PlaneAngleMeasure(45.0);
        assert_eq!(a.case_num(), 2);
        assert_eq!(a.plane_angle_measure(), Some(45.0));
    }
}
