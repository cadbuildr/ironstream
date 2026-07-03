// FILE: b_rep_check.rs
// occt: BRepCheck

/// This package provides tools to check the validity of the BRep.
pub struct BRepCheck;

impl BRepCheck {
    /// Add a status to a list
    pub fn add(list: &mut Vec<u32>, stat: u32) {
        if !list.contains(&stat) {
            list.push(stat);
        }
    }

    /// Print a status value
    pub fn print(stat: u32) -> String {
        match stat {
            0 => "BRepCheck_NoError".to_string(),
            1 => "BRepCheck_InvalidPointOnCurve".to_string(),
            2 => "BRepCheck_InvalidPointOnCurveOnSurface".to_string(),
            3 => "BRepCheck_InvalidPointOnSurface".to_string(),
            4 => "BRepCheck_No3DCurve".to_string(),
            5 => "BRepCheck_Multiple3DCurve".to_string(),
            6 => "BRepCheck_Invalid3DCurve".to_string(),
            7 => "BRepCheck_NoCurveOnSurface".to_string(),
            8 => "BRepCheck_InvalidCurveOnSurface".to_string(),
            9 => "BRepCheck_InvalidCurveOnClosedSurface".to_string(),
            10 => "BRepCheck_InvalidSameRangeFlag".to_string(),
            11 => "BRepCheck_InvalidSameParameterFlag".to_string(),
            12 => "BRepCheck_InvalidDegeneratedFlag".to_string(),
            13 => "BRepCheck_FreeEdge".to_string(),
            14 => "BRepCheck_InvalidMultiConnexity".to_string(),
            15 => "BRepCheck_InvalidRange".to_string(),
            16 => "BRepCheck_EmptySurface".to_string(),
            17 => "BRepCheck_SelfIntersectingWire".to_string(),
            18 => "BRepCheck_BadOrientation".to_string(),
            19 => "BRepCheck_BadOrientationOfClosedWire".to_string(),
            20 => "BRepCheck_NotClosedWire".to_string(),
            21 => "BRepCheck_NotConnectedWires".to_string(),
            22 => "BRepCheck_SubshapeNotInShape".to_string(),
            23 => "BRepCheck_BadLocationInShape".to_string(),
            24 => "BRepCheck_InvalidPolygonOnTriangulation".to_string(),
            25 => "BRepCheck_InvalidToleranceValue".to_string(),
            26 => "BRepCheck_EnclosedRegion".to_string(),
            27 => "BRepCheck_NotSewed".to_string(),
            28 => "BRepCheck_UnorientableShape".to_string(),
            29 => "BRepCheck_NotSolid".to_string(),
            30 => "BRepCheck_AlreadyChecked".to_string(),
            _ => "BRepCheck_Unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_list() {
        let mut list = Vec::new();
        BRepCheck::add(&mut list, 0);
        BRepCheck::add(&mut list, 1);
        assert_eq!(list.len(), 2);
        assert!(list.contains(&0));
        assert!(list.contains(&1));
    }

    #[test]
    fn test_no_duplicates() {
        let mut list = Vec::new();
        BRepCheck::add(&mut list, 5);
        BRepCheck::add(&mut list, 5);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_print_status() {
        assert_eq!(BRepCheck::print(0), "BRepCheck_NoError");
        assert_eq!(BRepCheck::print(4), "BRepCheck_No3DCurve");
        assert_eq!(BRepCheck::print(17), "BRepCheck_SelfIntersectingWire");
    }
}
