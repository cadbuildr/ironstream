// FILE: xml_obj_mgt_gp.rs
// occt: XmlObjMgt_GP

/// XmlObjMgt_GP provides translation services for geometric primitive objects.
/// Translates between gp (geometric primitive) objects and their string representations.
pub struct XmlObjMgt_GP;

impl XmlObjMgt_GP {
    /// Translate a gp_XYZ point to string representation.
    /// Format: "x y z"
    pub fn translate_xyz(x: f64, y: f64, z: f64) -> String {
        format!("{} {} {}", x, y, z)
    }

    /// Parse a gp_XYZ point from string representation.
    /// Format: "x y z"
    pub fn parse_xyz(s: &str) -> Option<(f64, f64, f64)> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return None;
        }

        let x = parts[0].parse::<f64>().ok()?;
        let y = parts[1].parse::<f64>().ok()?;
        let z = parts[2].parse::<f64>().ok()?;

        Some((x, y, z))
    }

    /// Translate a 3x3 matrix (gp_Mat) to string representation.
    /// Format: "m00 m01 m02 m10 m11 m12 m20 m21 m22"
    pub fn translate_mat(mat: &[[f64; 3]; 3]) -> String {
        let mut result = String::new();
        for row in mat.iter() {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(&val.to_string());
            }
            result.push(' ');
        }
        result.trim().to_string()
    }

    /// Parse a 3x3 matrix (gp_Mat) from string representation.
    pub fn parse_mat(s: &str) -> Option<[[f64; 3]; 3]> {
        let parts: Vec<f64> = s.split_whitespace()
            .map(|p| p.parse::<f64>())
            .collect::<Result<_, _>>()
            .ok()?;

        if parts.len() != 9 {
            return None;
        }

        Some([
            [parts[0], parts[1], parts[2]],
            [parts[3], parts[4], parts[5]],
            [parts[6], parts[7], parts[8]],
        ])
    }

    /// Translate a gp_Trsf (transformation) to string representation.
    /// Simplified: represents as 4x3 matrix (3x3 rotation + translation)
    pub fn translate_trsf(rot: &[[f64; 3]; 3], trans: &[f64; 3]) -> String {
        let mut result = String::new();
        for row in rot.iter() {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(&val.to_string());
            }
            result.push(' ');
        }
        for (i, val) in trans.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&val.to_string());
        }
        result
    }

    /// Parse a gp_Trsf from string representation.
    pub fn parse_trsf(s: &str) -> Option<([[f64; 3]; 3], [f64; 3])> {
        let parts: Vec<f64> = s.split_whitespace()
            .map(|p| p.parse::<f64>())
            .collect::<Result<_, _>>()
            .ok()?;

        if parts.len() != 12 {
            return None;
        }

        let rot = [
            [parts[0], parts[1], parts[2]],
            [parts[3], parts[4], parts[5]],
            [parts[6], parts[7], parts[8]],
        ];

        let trans = [parts[9], parts[10], parts[11]];

        Some((rot, trans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_xyz() {
        let s = XmlObjMgt_GP::translate_xyz(1.0, 2.0, 3.0);
        assert_eq!(s, "1 2 3");
    }

    #[test]
    fn test_parse_xyz() {
        let (x, y, z) = XmlObjMgt_GP::parse_xyz("1.5 2.5 3.5").unwrap();
        assert_eq!(x, 1.5);
        assert_eq!(y, 2.5);
        assert_eq!(z, 3.5);
    }

    #[test]
    fn test_translate_mat() {
        let mat = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let s = XmlObjMgt_GP::translate_mat(&mat);
        assert_eq!(s, "1 2 3 4 5 6 7 8 9");
    }

    #[test]
    fn test_parse_mat() {
        let mat = XmlObjMgt_GP::parse_mat("1 2 3 4 5 6 7 8 9").unwrap();
        assert_eq!(mat[0][0], 1.0);
        assert_eq!(mat[2][2], 9.0);
    }

    #[test]
    fn test_translate_trsf() {
        let rot = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let trans = [1.0, 2.0, 3.0];
        let s = XmlObjMgt_GP::translate_trsf(&rot, &trans);
        assert!(s.contains("1"));
        assert!(s.contains("2"));
        assert!(s.contains("3"));
    }

    #[test]
    fn test_parse_trsf() {
        let s = "1 0 0 0 1 0 0 0 1 1 2 3";
        let (rot, trans) = XmlObjMgt_GP::parse_trsf(s).unwrap();
        assert_eq!(rot[0][0], 1.0);
        assert_eq!(trans[0], 1.0);
    }
}
