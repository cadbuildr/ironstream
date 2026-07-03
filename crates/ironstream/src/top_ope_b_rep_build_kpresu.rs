// FILE: top_ope_b_rep_build_kpresu.rs
// occt: TopOpeBRepBuild_kpresu

/// RESUNDEF: resultat indefini
pub const RESUNDEF: i32 = -100;

/// RESNEWSHA2: nouveau shape de meme type
pub const RESNEWSHA2: i32 = -12;

/// RESNEWSHA1: nouveau shape de meme type
pub const RESNEWSHA1: i32 = -11;

/// RESNEWCOM: nouveau compound
pub const RESNEWCOM: i32 = -3;

/// RESNEWSOL: nouveau solide
pub const RESNEWSOL: i32 = -2;

/// RESNEWSHE: nouveau shell
pub const RESNEWSHE: i32 = -1;

/// RESNULL: resultat = vide
pub const RESNULL: i32 = 0;

/// RESSHAPE1: resultat = shell accedant face tangente de 1
pub const RESSHAPE1: i32 = 1;

/// RESSHAPE2: resultat = shell accedant face tangente de 2
pub const RESSHAPE2: i32 = 2;

/// RESSHAPE12: resultat = shapes 1 et 2
pub const RESSHAPE12: i32 = 3;

/// RESFACE1: resultat = face tangente de 1
pub const RESFACE1: i32 = 11;

/// RESFACE2: resultat = face tangente de 2
pub const RESFACE2: i32 = 12;

/// SHEUNDEF: indefini
pub const SHEUNDEF: i32 = -100;

/// SHEAUCU: ne prendre ni classifier aucun shell
pub const SHEAUCU: i32 = -1;

/// SHECLASCOUR: classifier le shell courant
pub const SHECLASCOUR: i32 = 1;

/// SHECLASAUTR: classifier tous les autres shells
pub const SHECLASAUTR: i32 = 2;

/// SHECLASTOUS: classifier tous les shells
pub const SHECLASTOUS: i32 = 3;

/// SHEGARDCOUR: prendre le shell courant sans classifier
pub const SHEGARDCOUR: i32 = 4;

/// SHEGARDAUTR: prendre tous les autres shells sans classifier
pub const SHEGARDAUTR: i32 = 5;

/// SHEGARDTOUS: prendre tous les shells sans classifier
pub const SHEGARDTOUS: i32 = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_constants() {
        assert_eq!(RESUNDEF, -100);
        assert_eq!(RESNEWSHA2, -12);
        assert_eq!(RESNEWSHA1, -11);
        assert_eq!(RESNEWCOM, -3);
        assert_eq!(RESNEWSOL, -2);
        assert_eq!(RESNEWSHE, -1);
        assert_eq!(RESNULL, 0);
        assert_eq!(RESSHAPE1, 1);
        assert_eq!(RESSHAPE2, 2);
        assert_eq!(RESSHAPE12, 3);
        assert_eq!(RESFACE1, 11);
        assert_eq!(RESFACE2, 12);
    }

    #[test]
    fn test_shell_constants() {
        assert_eq!(SHEUNDEF, -100);
        assert_eq!(SHEAUCU, -1);
        assert_eq!(SHECLASCOUR, 1);
        assert_eq!(SHECLASAUTR, 2);
        assert_eq!(SHECLASTOUS, 3);
        assert_eq!(SHEGARDCOUR, 4);
        assert_eq!(SHEGARDAUTR, 5);
        assert_eq!(SHEGARDTOUS, 6);
    }
}
