// FILE: intf_pi_type.rs
// occt: Intf_PIType

/// Polygon interference type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIType {
    Open,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_type() {
        let _t1 = PIType::Open;
        let _t2 = PIType::Closed;
        assert_ne!(PIType::Open, PIType::Closed);
    }
}
