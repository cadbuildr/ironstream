// FILE: int_surf_type_trans.rs
// occt: IntSurf_TypeTrans

/// Type of surface transition
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeTrans {
    In,
    Out,
    Touch,
    Undecided,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_trans() {
        let _t1 = TypeTrans::In;
        let _t2 = TypeTrans::Out;
        let _t3 = TypeTrans::Touch;
        let _t4 = TypeTrans::Undecided;
        assert_ne!(TypeTrans::In, TypeTrans::Out);
    }
}
