// FILE: b_rep_feat_status_error.rs
// occt: BRepFeat_StatusError

/// Enumeration describing errors in feature operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BRepFeatStatusError {
    Ok = 0,
    BadDirect = 1,
    BadIntersect = 2,
    EmptyBaryCurve = 3,
    EmptyCutResult = 4,
    FalseSide = 5,
    IncDirection = 6,
    IncSlidFace = 7,
    IncParameter = 8,
    IncTypes = 9,
    IntervalOverlap = 10,
    InvFirstShape = 11,
    InvOption = 12,
    InvShape = 13,
    LocOpeNotDone = 14,
    LocOpeInvNotDone = 15,
    NoExtFace = 16,
    NoFaceProf = 17,
    NoGluer = 18,
    NoIntersectF = 19,
    NoIntersectU = 20,
    NoParts = 21,
    NoProjPt = 22,
    NotInitialized = 23,
    NotYetImplemented = 24,
    NullRealTool = 25,
    NullToolF = 26,
    NullToolU = 27,
}

impl BRepFeatStatusError {
    /// Converts an integer to a status error.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(BRepFeatStatusError::Ok),
            1 => Some(BRepFeatStatusError::BadDirect),
            2 => Some(BRepFeatStatusError::BadIntersect),
            3 => Some(BRepFeatStatusError::EmptyBaryCurve),
            4 => Some(BRepFeatStatusError::EmptyCutResult),
            5 => Some(BRepFeatStatusError::FalseSide),
            6 => Some(BRepFeatStatusError::IncDirection),
            7 => Some(BRepFeatStatusError::IncSlidFace),
            8 => Some(BRepFeatStatusError::IncParameter),
            9 => Some(BRepFeatStatusError::IncTypes),
            10 => Some(BRepFeatStatusError::IntervalOverlap),
            11 => Some(BRepFeatStatusError::InvFirstShape),
            12 => Some(BRepFeatStatusError::InvOption),
            13 => Some(BRepFeatStatusError::InvShape),
            14 => Some(BRepFeatStatusError::LocOpeNotDone),
            15 => Some(BRepFeatStatusError::LocOpeInvNotDone),
            16 => Some(BRepFeatStatusError::NoExtFace),
            17 => Some(BRepFeatStatusError::NoFaceProf),
            18 => Some(BRepFeatStatusError::NoGluer),
            19 => Some(BRepFeatStatusError::NoIntersectF),
            20 => Some(BRepFeatStatusError::NoIntersectU),
            21 => Some(BRepFeatStatusError::NoParts),
            22 => Some(BRepFeatStatusError::NoProjPt),
            23 => Some(BRepFeatStatusError::NotInitialized),
            24 => Some(BRepFeatStatusError::NotYetImplemented),
            25 => Some(BRepFeatStatusError::NullRealTool),
            26 => Some(BRepFeatStatusError::NullToolF),
            27 => Some(BRepFeatStatusError::NullToolU),
            _ => None,
        }
    }

    /// Converts a status error to an integer.
    pub fn to_int(&self) -> i32 {
        *self as i32
    }

    /// Returns a string description of the error.
    pub fn description(&self) -> &'static str {
        match self {
            BRepFeatStatusError::Ok => "OK",
            BRepFeatStatusError::BadDirect => "Bad Direction",
            BRepFeatStatusError::BadIntersect => "Bad Intersection",
            BRepFeatStatusError::EmptyBaryCurve => "Empty Bary Curve",
            BRepFeatStatusError::EmptyCutResult => "Empty Cut Result",
            BRepFeatStatusError::FalseSide => "False Side",
            BRepFeatStatusError::IncDirection => "Inconsistent Direction",
            BRepFeatStatusError::IncSlidFace => "Inconsistent Slid Face",
            BRepFeatStatusError::IncParameter => "Inconsistent Parameter",
            BRepFeatStatusError::IncTypes => "Inconsistent Types",
            BRepFeatStatusError::IntervalOverlap => "Interval Overlap",
            BRepFeatStatusError::InvFirstShape => "Invalid First Shape",
            BRepFeatStatusError::InvOption => "Invalid Option",
            BRepFeatStatusError::InvShape => "Invalid Shape",
            BRepFeatStatusError::LocOpeNotDone => "Local Operation Not Done",
            BRepFeatStatusError::LocOpeInvNotDone => "Local Operation Invalid Not Done",
            BRepFeatStatusError::NoExtFace => "No Extended Face",
            BRepFeatStatusError::NoFaceProf => "No Face Profile",
            BRepFeatStatusError::NoGluer => "No Gluer",
            BRepFeatStatusError::NoIntersectF => "No Intersection F",
            BRepFeatStatusError::NoIntersectU => "No Intersection U",
            BRepFeatStatusError::NoParts => "No Parts",
            BRepFeatStatusError::NoProjPt => "No Projected Point",
            BRepFeatStatusError::NotInitialized => "Not Initialized",
            BRepFeatStatusError::NotYetImplemented => "Not Yet Implemented",
            BRepFeatStatusError::NullRealTool => "Null Real Tool",
            BRepFeatStatusError::NullToolF => "Null Tool F",
            BRepFeatStatusError::NullToolU => "Null Tool U",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_error_enum() {
        assert_eq!(BRepFeatStatusError::Ok.to_int(), 0);
        assert_eq!(BRepFeatStatusError::BadDirect.to_int(), 1);
        assert_eq!(BRepFeatStatusError::NullToolU.to_int(), 27);
    }

    #[test]
    fn test_status_error_from_int() {
        assert_eq!(
            BRepFeatStatusError::from_int(0),
            Some(BRepFeatStatusError::Ok)
        );
        assert_eq!(
            BRepFeatStatusError::from_int(28),
            None
        );
    }

    #[test]
    fn test_status_error_description() {
        assert_eq!(BRepFeatStatusError::Ok.description(), "OK");
        assert_eq!(BRepFeatStatusError::BadDirect.description(), "Bad Direction");
    }
}
