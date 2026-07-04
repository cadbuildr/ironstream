// FILE: step_basic_size_select.rs
// occt: StepBasic_SizeSelect

#[derive(Clone, Debug)]
pub enum StepBasicSizeSelect {
    PosLength(f64),
    NegLength(f64),
    PosPlaneAngle(f64),
    NegPlaneAngle(f64),
}

impl StepBasicSizeSelect {
    pub fn case_num(&self) -> i32 {
        match self {
            Self::PosLength(_) => 1,
            Self::NegLength(_) => 2,
            Self::PosPlaneAngle(_) => 3,
            Self::NegPlaneAngle(_) => 4,
        }
    }

    pub fn pos_length(&self) -> Option<f64> {
        match self { Self::PosLength(v) => Some(*v), _ => None }
    }

    pub fn neg_length(&self) -> Option<f64> {
        match self { Self::NegLength(v) => Some(*v), _ => None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        let p = StepBasicSizeSelect::PosLength(5.0);
        assert_eq!(p.case_num(), 1);
        assert_eq!(p.pos_length(), Some(5.0));

        let n = StepBasicSizeSelect::NegLength(-3.0);
        assert_eq!(n.case_num(), 2);
    }
}
