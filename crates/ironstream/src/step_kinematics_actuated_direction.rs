// FILE: step_kinematics_actuated_direction.rs
// occt: StepKinematics_ActuatedDirection

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActuatedDirection {
    Bidirectional,
    PositiveOnly,
    NegativeOnly,
    NotActuated,
}

impl ActuatedDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActuatedDirection::Bidirectional => "Bidirectional",
            ActuatedDirection::PositiveOnly => "PositiveOnly",
            ActuatedDirection::NegativeOnly => "NegativeOnly",
            ActuatedDirection::NotActuated => "NotActuated",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuated_direction_variants() {
        let bd = ActuatedDirection::Bidirectional;
        let po = ActuatedDirection::PositiveOnly;
        let no = ActuatedDirection::NegativeOnly;
        let na = ActuatedDirection::NotActuated;

        assert_eq!(bd.as_str(), "Bidirectional");
        assert_eq!(po.as_str(), "PositiveOnly");
        assert_eq!(no.as_str(), "NegativeOnly");
        assert_eq!(na.as_str(), "NotActuated");
    }

    #[test]
    fn test_actuated_direction_equality() {
        let dir1 = ActuatedDirection::Bidirectional;
        let dir2 = ActuatedDirection::Bidirectional;
        assert_eq!(dir1, dir2);
    }

    #[test]
    fn test_actuated_direction_copy() {
        let dir = ActuatedDirection::PositiveOnly;
        let dir_copy = dir;
        assert_eq!(dir, dir_copy);
    }
}
