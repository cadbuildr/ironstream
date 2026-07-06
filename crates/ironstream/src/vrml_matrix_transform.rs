// FILE: vrml_matrix_transform.rs
// occt: Vrml_MatrixTransform
//
// Faithful port of OCCT Vrml_MatrixTransform (DataExchange/TKDEVRML/Vrml/
// Vrml_MatrixTransform.hxx): the VRML 1.0 `MatrixTransform` node, storing
// a 4x4 transformation matrix. Default is the identity matrix. Print emits
// the matrix as 16 space-separated values.

/// Real formatter matching C++ defaultfloat (printf "%g").
fn vrml_matrix_transform_real(v: f64) -> String {
    let p = 6usize;
    let sci = format!("{:.*e}", p - 1, v);
    let epos = sci.find('e').expect("exponent");
    let exp: i32 = sci[epos + 1..].parse().expect("exp digits");
    if exp < -4 || exp >= p as i32 {
        let mant = sci[..epos].trim_end_matches('0').trim_end_matches('.');
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { '-' } else { '+' },
            exp.abs()
        )
    } else {
        let prec = (p as i32 - 1 - exp).max(0) as usize;
        let fixed = format!("{:.*}", prec, v);
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}

/// Port of Vrml_MatrixTransform. Stores a 4x4 matrix in row-major order.
#[derive(Debug, Clone, PartialEq)]
pub struct VrmlMatrixTransform {
    // 4x4 matrix stored as [row0, row1, row2, row3], each row is [a, b, c, d]
    my_matrix: [[f64; 4]; 4],
}

impl VrmlMatrixTransform {
    /// Vrml_MatrixTransform(): identity matrix.
    pub fn new() -> Self {
        VrmlMatrixTransform {
            my_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Vrml_MatrixTransform(const gp_Trsf&) - from a transformation matrix.
    /// For now, accept raw array.
    pub fn with_matrix(matrix: [[f64; 4]; 4]) -> Self {
        VrmlMatrixTransform { my_matrix: matrix }
    }

    pub fn set_matrix(&mut self, matrix: [[f64; 4]; 4]) {
        self.my_matrix = matrix;
    }

    pub fn matrix(&self) -> &[[f64; 4]; 4] {
        &self.my_matrix
    }

    /// Standard_OStream& Print(Standard_OStream&) const.
    pub fn print(&self, an_ostream: &mut String) {
        an_ostream.push_str("MatrixTransform {\n");
        an_ostream.push_str("    matrix\t");

        // Print matrix as 16 space-separated values (row-major)
        for (i, row) in self.my_matrix.iter().enumerate() {
            if i > 0 {
                an_ostream.push_str("\n\t\t");
            }
            for (j, &val) in row.iter().enumerate() {
                if j > 0 {
                    an_ostream.push(' ');
                }
                an_ostream.push_str(&vrml_matrix_transform_real(val));
            }
        }

        an_ostream.push_str("\n}\n");
    }
}

impl Default for VrmlMatrixTransform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix() {
        let mt = VrmlMatrixTransform::new();
        let m = mt.matrix();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[2][2], 1.0);
        assert_eq!(m[3][3], 1.0);
        assert_eq!(m[0][1], 0.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[0][3], 0.0);
    }

    #[test]
    fn print_identity_matrix() {
        let mt = VrmlMatrixTransform::new();
        let mut out = String::new();
        mt.print(&mut out);
        assert!(out.contains("MatrixTransform {"));
        assert!(out.contains("matrix"));
        // Should have 16 values
        let values: Vec<&str> = out.split_whitespace().collect();
        // Count numeric values (excluding keywords and braces)
        let numeric_count = values
            .iter()
            .filter(|v| v.parse::<f64>().is_ok())
            .count();
        assert_eq!(numeric_count, 16);
    }

    #[test]
    fn custom_translation_matrix() {
        let matrix = [
            [1.0, 0.0, 0.0, 5.0],  // translate x=5
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let mt = VrmlMatrixTransform::with_matrix(matrix);
        let mut out = String::new();
        mt.print(&mut out);
        assert!(out.contains("5")); // Should have the translation value
    }

    #[test]
    fn setter() {
        let mut mt = VrmlMatrixTransform::new();
        let matrix = [
            [2.0, 0.0, 0.0, 0.0],  // scale by 2
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        mt.set_matrix(matrix);
        assert_eq!(mt.matrix()[0][0], 2.0);
    }
}
