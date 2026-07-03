// FILE: int_res2d_position.rs
// occt: IntRes2d_Position

/// Position enumeration for 2D curves
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Head,
    Middle,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_variants() {
        let _p1 = Position::Head;
        let _p2 = Position::Middle;
        let _p3 = Position::End;
        assert_ne!(Position::Head, Position::Middle);
        assert_ne!(Position::Middle, Position::End);
    }
}
