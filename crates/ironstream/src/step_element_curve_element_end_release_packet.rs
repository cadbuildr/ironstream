// FILE: step_element_curve_element_end_release_packet.rs
// occt: StepElement_CurveElementEndReleasePacket

pub struct CurveElementEndReleasePacket {
    pub release_freedom: Vec<String>,
}

impl CurveElementEndReleasePacket {
    pub fn new() -> Self {
        CurveElementEndReleasePacket {
            release_freedom: Vec::new(),
        }
    }

    pub fn add_release_freedom(&mut self, freedom: String) {
        self.release_freedom.push(freedom);
    }

    pub fn get_release_freedom(&self) -> &[String] {
        &self.release_freedom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let packet = CurveElementEndReleasePacket::new();
        assert!(packet.release_freedom.is_empty());
    }

    #[test]
    fn test_add_release_freedom() {
        let mut packet = CurveElementEndReleasePacket::new();
        packet.add_release_freedom("freedom1".to_string());
        packet.add_release_freedom("freedom2".to_string());
        assert_eq!(packet.get_release_freedom().len(), 2);
    }
}
