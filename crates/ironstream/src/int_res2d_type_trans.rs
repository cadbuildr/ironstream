// FILE: int_res2d_type_trans.rs
// occt: IntRes2d_TypeTrans

/// Type of transition enumeration
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
    fn test_type_trans_variants() {
        let _t1 = TypeTrans::In;
        let _t2 = TypeTrans::Out;
        let _t3 = TypeTrans::Touch;
        let _t4 = TypeTrans::Undecided;
        assert_ne!(TypeTrans::In, TypeTrans::Out);
        assert_ne!(TypeTrans::Touch, TypeTrans::Undecided);
    }
}
