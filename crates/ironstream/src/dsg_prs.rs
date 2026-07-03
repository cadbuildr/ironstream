// FILE: dsg_prs.rs
// occt: DsgPrs

pub mod dsg_prs {
    pub fn compute_text_offset(text_width: f64, text_height: f64, arrow_width: f64) -> [f64; 2] {
        [(text_width + arrow_width) / 2.0, text_height / 2.0]
    }
}

#[cfg(test)]
mod tests {
    use super::dsg_prs::*;
    #[test]
    fn test_offset() {
        let offset = compute_text_offset(10.0, 5.0, 2.0);
        assert_eq!(offset[0], 6.0);
    }
}
