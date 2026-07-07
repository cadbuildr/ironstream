// FILE: graphic3d_texture_unit.rs
// occt: Graphic3d_TextureUnit

/// Texture unit enumeration.
///
/// In OCCT this is a plain C++ enum where aliases share the numeric value of
/// the unit they refer to (e.g. Graphic3d_TextureUnit_BaseColor ==
/// Graphic3d_TextureUnit_0), so comparison traits are implemented over
/// `as_index()` rather than derived over variants.
#[derive(Debug, Clone, Copy)]
pub enum TextureUnit {
    // value as index number
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
    Unit8,
    Unit9,
    Unit10,
    Unit11,
    Unit12,
    Unit13,
    Unit14,
    Unit15,

    // aliases

    // sampler2D occSamplerBaseColor.
    // RGB(A) base color of the material and alpha mask/opacity.
    // Alias for Unit0
    BaseColor,

    // sampler2D occSamplerEmissive.
    // RGB emissive map controls the color and intensity of the light being emitted by the material.
    // Alias for Unit1
    Emissive,

    // sampler2D occSamplerOcclusion.
    // Occlusion map indicating areas of indirect lighting.
    // Encoded into RED channel, with 1.0 meaning no occlusion (full color intensity) and 0.0 complete occlusion.
    // Alias for Unit2
    Occlusion,

    // sampler2D occSamplerNormal.
    // XYZ tangent space normal map.
    // Alias for Unit3
    Normal,

    // sampler2D occSamplerMetallicRoughness.
    // Metalness + roughness of the material.
    // Encoded into GREEN (roughness) + BLUE (metallic) channels.
    // Alias for Unit4
    MetallicRoughness,

    // samplerCube occSampler0.
    // Environment cubemap for background. Rendered by dedicated program and normally occupies first texture unit.
    // Alias for Unit0
    EnvMap,

    // sampler2D occSamplerPointSprite.
    // Sprite alpha-mask or RGBA image mapped using point UV, additional to BaseColor.
    // Alias for Unit1
    PointSprite,

    // sampler2D occDepthPeelingDepth.
    // 1st texture unit for Depth Peeling lookups.
    DepthPeelingDepth,

    // sampler2D occDepthPeelingFrontColor.
    // 2nd texture unit for Depth Peeling lookups.
    DepthPeelingFrontColor,

    // sampler2D occShadowMapSampler.
    // Directional light source shadowmap texture.
    ShadowMap,

    // sampler2D occEnvLUT.
    // Lookup table for approximated PBR environment lighting.
    // Configured as index at the end of available texture units - 3.
    PbrEnvironmentLUT,

    // sampler2D occDiffIBLMapSHCoeffs.
    // Diffuse (irradiance) IBL map's spherical harmonics coefficients baked for PBR.
    // Configured as index at the end of available texture units - 2.
    PbrIblDiffuseSH,

    // samplerCube occSpecIBLMap.
    // Specular IBL (Image-Based Lighting) environment map baked for PBR from environment cubemap.
    // Configured as index at the end of available texture units - 1.
    PbrIblSpecular,
}

impl TextureUnit {
    /// Returns the numeric index of the texture unit.
    pub fn as_index(&self) -> i32 {
        match self {
            TextureUnit::Unit0 => 0,
            TextureUnit::Unit1 => 1,
            TextureUnit::Unit2 => 2,
            TextureUnit::Unit3 => 3,
            TextureUnit::Unit4 => 4,
            TextureUnit::Unit5 => 5,
            TextureUnit::Unit6 => 6,
            TextureUnit::Unit7 => 7,
            TextureUnit::Unit8 => 8,
            TextureUnit::Unit9 => 9,
            TextureUnit::Unit10 => 10,
            TextureUnit::Unit11 => 11,
            TextureUnit::Unit12 => 12,
            TextureUnit::Unit13 => 13,
            TextureUnit::Unit14 => 14,
            TextureUnit::Unit15 => 15,
            // Aliases
            TextureUnit::BaseColor => 0,
            TextureUnit::Emissive => 1,
            TextureUnit::Occlusion => 2,
            TextureUnit::Normal => 3,
            TextureUnit::MetallicRoughness => 4,
            TextureUnit::EnvMap => 0,
            TextureUnit::PointSprite => 1,
            // Reserved units for special purposes (negative indices)
            TextureUnit::DepthPeelingDepth => -6,
            TextureUnit::DepthPeelingFrontColor => -5,
            TextureUnit::ShadowMap => -4,
            TextureUnit::PbrEnvironmentLUT => -3,
            TextureUnit::PbrIblDiffuseSH => -2,
            TextureUnit::PbrIblSpecular => -1,
        }
    }

    /// Number of regular texture units (0-15)
    pub const NB: usize = 16;
}

impl PartialEq for TextureUnit {
    fn eq(&self, other: &Self) -> bool {
        self.as_index() == other.as_index()
    }
}

impl Eq for TextureUnit {}

impl std::hash::Hash for TextureUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_index().hash(state);
    }
}

impl PartialOrd for TextureUnit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TextureUnit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_index().cmp(&other.as_index())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_indices() {
        assert_eq!(TextureUnit::Unit0.as_index(), 0);
        assert_eq!(TextureUnit::Unit1.as_index(), 1);
        assert_eq!(TextureUnit::Unit5.as_index(), 5);
        assert_eq!(TextureUnit::Unit10.as_index(), 10);
        assert_eq!(TextureUnit::Unit15.as_index(), 15);
    }

    #[test]
    fn test_semantic_aliases() {
        assert_eq!(TextureUnit::BaseColor.as_index(), 0);
        assert_eq!(TextureUnit::Emissive.as_index(), 1);
        assert_eq!(TextureUnit::Occlusion.as_index(), 2);
        assert_eq!(TextureUnit::Normal.as_index(), 3);
        assert_eq!(TextureUnit::MetallicRoughness.as_index(), 4);
    }

    #[test]
    fn test_env_and_sprite_aliases() {
        assert_eq!(TextureUnit::EnvMap.as_index(), 0);
        assert_eq!(TextureUnit::PointSprite.as_index(), 1);
    }

    #[test]
    fn test_depth_peeling_units() {
        assert_eq!(TextureUnit::DepthPeelingDepth.as_index(), -6);
        assert_eq!(TextureUnit::DepthPeelingFrontColor.as_index(), -5);
    }

    #[test]
    fn test_shadow_and_pbr_units() {
        assert_eq!(TextureUnit::ShadowMap.as_index(), -4);
        assert_eq!(TextureUnit::PbrEnvironmentLUT.as_index(), -3);
        assert_eq!(TextureUnit::PbrIblDiffuseSH.as_index(), -2);
        assert_eq!(TextureUnit::PbrIblSpecular.as_index(), -1);
    }

    #[test]
    fn test_nb_constant() {
        assert_eq!(TextureUnit::NB, 16);
    }

    #[test]
    fn test_all_units_unique_indices() {
        let units = [
            TextureUnit::Unit0,
            TextureUnit::Unit1,
            TextureUnit::Unit2,
            TextureUnit::Unit3,
            TextureUnit::Unit4,
            TextureUnit::Unit5,
            TextureUnit::Unit6,
            TextureUnit::Unit7,
            TextureUnit::Unit8,
            TextureUnit::Unit9,
            TextureUnit::Unit10,
            TextureUnit::Unit11,
            TextureUnit::Unit12,
            TextureUnit::Unit13,
            TextureUnit::Unit14,
            TextureUnit::Unit15,
        ];

        for (i, unit) in units.iter().enumerate() {
            assert_eq!(unit.as_index(), i as i32);
        }
    }

    #[test]
    fn test_eq() {
        assert_eq!(TextureUnit::BaseColor, TextureUnit::Unit0);
        assert_ne!(TextureUnit::BaseColor, TextureUnit::Unit1);
        assert_eq!(TextureUnit::Emissive, TextureUnit::Unit1);
    }
}
