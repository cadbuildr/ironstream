// FILE: xcaf_dim_tol_objects_dimension_grade.rs
// occt: XCAFDimTolObjects_DimensionGrade

/// Enumeration for dimension grades (IT quality grades).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GradeType {
    IT01,
    IT0,
    IT1,
    IT2,
    IT3,
    IT4,
    IT5,
    IT6,
    IT7,
    IT8,
    IT9,
    IT10,
    IT11,
    IT12,
    IT13,
    IT14,
    IT15,
    IT16,
}

#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DimensionGrade {
    grade: GradeType,
}

impl XCAFDimTolObjects_DimensionGrade {
    /// Create a new dimension grade.
    pub fn new(grade: GradeType) -> Self {
        Self { grade }
    }

    /// Get the grade.
    pub fn grade(&self) -> &GradeType {
        &self.grade
    }

    /// Set the grade.
    pub fn set_grade(&mut self, grade: GradeType) {
        self.grade = grade;
    }
}

impl Default for XCAFDimTolObjects_DimensionGrade {
    fn default() -> Self {
        Self::new(GradeType::IT7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_grade() {
        let grade = XCAFDimTolObjects_DimensionGrade::new(GradeType::IT5);
        assert_eq!(grade.grade(), &GradeType::IT5);
    }

    #[test]
    fn test_set_grade() {
        let mut grade = XCAFDimTolObjects_DimensionGrade::new(GradeType::IT6);
        grade.set_grade(GradeType::IT8);
        assert_eq!(grade.grade(), &GradeType::IT8);
    }

    #[test]
    fn test_default() {
        let grade = XCAFDimTolObjects_DimensionGrade::default();
        assert_eq!(grade.grade(), &GradeType::IT7);
    }

    #[test]
    fn test_grade_comparison() {
        assert!(GradeType::IT5 < GradeType::IT7);
        assert!(GradeType::IT10 > GradeType::IT5);
    }
}
