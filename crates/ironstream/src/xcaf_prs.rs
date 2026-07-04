// FILE: xcaf_prs.rs
// occt: XCAFPrs
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! Presentation (visualisation, selection etc.) tools for DECAF documents.
//!
//! Port of OCCT `XCAFPrs` (src/DataExchange/TKXCAF/XCAFPrs/XCAFPrs.{hxx,cxx}).
//!
//! The C++ class exposes:
//!   - a static view-name-mode flag (`SetViewNameMode` / `GetViewNameMode`)
//!     backed by the file-static `viewnameMode` (initialised to `false`);
//!   - the static `CollectStyleSettings` helper which walks a label hierarchy
//!     and fills a shape -> style map (colors from the color tool, visibility,
//!     layer color when "color by layer" is set).
//!
//! The full `CollectStyleSettings` requires the whole XCAFDoc machinery; here
//! we model the document as a minimal local label tree carrying optional
//! style inputs and port the traversal/merge order faithfully:
//! references first, then components, then subshapes followed by the label
//! itself, with later bindings overriding earlier ones for the same shape key.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

/// Static flag mirroring the file-static `viewnameMode` in XCAFPrs.cxx.
static VIEW_NAME_MODE: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Minimal local models of OCCT dependencies
// ---------------------------------------------------------------------------

/// Minimal stand-in for Quantity_ColorRGBA.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity_ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Quantity_ColorRGBA {
    pub const WHITE: Quantity_ColorRGBA = Quantity_ColorRGBA {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
}

/// Minimal stand-in for XCAFPrs_Style: surface/curve colors + visibility.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XCAFPrs_Style {
    pub color_surf: Option<Quantity_ColorRGBA>,
    pub color_curv: Option<Quantity_ColorRGBA>,
    pub visible: bool,
}

impl Default for XCAFPrs_Style {
    fn default() -> Self {
        XCAFPrs_Style {
            color_surf: None,
            color_curv: None,
            visible: true,
        }
    }
}

impl XCAFPrs_Style {
    /// Mirrors XCAFPrs_Style::IsEmpty: no colors set and visible.
    pub fn IsEmpty(&self) -> bool {
        self.color_surf.is_none() && self.color_curv.is_none() && self.visible
    }
}

/// Minimal stand-in for a TDF_Label of an XCAF document shape structure.
///
/// Each label owns a shape identifier (stand-in for TopoDS_Shape key),
/// optional color settings (XCAFDoc_ColorGen / ColorSurf / ColorCurv),
/// a visibility flag, an optional "color by layer" layer color, and
/// child structure: reference, components and subshapes.
#[derive(Debug, Clone, Default)]
pub struct TDF_Label {
    pub shape_id: String,
    pub color_gen: Option<Quantity_ColorRGBA>,
    pub color_surf: Option<Quantity_ColorRGBA>,
    pub color_curv: Option<Quantity_ColorRGBA>,
    pub visible: bool,
    pub is_color_by_layer: bool,
    /// Single-layer color override used when the referring label lies on
    /// exactly one layer that has a color (per the .cxx layer handling).
    pub layer_color: Option<Quantity_ColorRGBA>,
    pub reference: Option<Box<TDF_Label>>,
    pub components: Vec<TDF_Label>,
    pub subshapes: Vec<TDF_Label>,
}

impl TDF_Label {
    pub fn new(shape_id: &str) -> Self {
        TDF_Label {
            shape_id: shape_id.to_string(),
            visible: true,
            ..Default::default()
        }
    }
}

pub struct XCAFPrs;

impl XCAFPrs {
    /// Set ViewNameMode used to indicate whether to display names or not.
    pub fn SetViewNameMode(view_name_mode: bool) {
        VIEW_NAME_MODE.store(view_name_mode, Ordering::SeqCst);
    }

    /// Get current ViewNameMode (false by default, as in OCCT).
    pub fn GetViewNameMode() -> bool {
        VIEW_NAME_MODE.load(Ordering::SeqCst)
    }

    /// Collect styles defined for the shape on label `label` and its
    /// components and subshapes, filling a map of shape -> style.
    ///
    /// Traversal order ported from XCAFPrs::CollectStyleSettings:
    /// 1. referred shape first (with possible layer color override),
    /// 2. then assembly components,
    /// 3. then subshapes followed by the label itself; empty styles are
    ///    skipped, existing bindings are overridden by later ones.
    pub fn CollectStyleSettings(
        label: &TDF_Label,
        settings: &mut HashMap<String, XCAFPrs_Style>,
        layer_color: Quantity_ColorRGBA,
    ) {
        // For references, first collect colors of the referred shape.
        if let Some(ref referred) = label.reference {
            let mut sub_layer_color = layer_color;
            if let Some(lc) = label.layer_color {
                sub_layer_color = lc;
            }
            Self::CollectStyleSettings(referred, settings, sub_layer_color);
        }

        // For assemblies, first collect colors defined in components.
        for component in &label.components {
            Self::CollectStyleSettings(component, settings, layer_color);
        }

        // Collect settings on subshapes, and then the label itself.
        let mut labels: Vec<&TDF_Label> = label.subshapes.iter().collect();
        labels.push(label);
        for lab in labels {
            let mut style = XCAFPrs_Style {
                visible: lab.visible,
                ..Default::default()
            };

            if lab.is_color_by_layer {
                let color = lab.layer_color.unwrap_or(layer_color);
                style.color_curv = Some(color);
                style.color_surf = Some(color);
            } else {
                // fillStyleColors: ColorGen fills both, then Surf/Curv refine.
                if let Some(c) = lab.color_gen {
                    style.color_curv = Some(c);
                    style.color_surf = Some(c);
                }
                if let Some(c) = lab.color_surf {
                    style.color_surf = Some(c);
                }
                if let Some(c) = lab.color_curv {
                    style.color_curv = Some(c);
                }
            }

            if style.IsEmpty() {
                continue;
            }
            // Bind or override, as ChangeSeek/Add does in the .cxx.
            settings.insert(lab.shape_id.clone(), style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: Quantity_ColorRGBA = Quantity_ColorRGBA {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    const GREEN: Quantity_ColorRGBA = Quantity_ColorRGBA {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    #[test]
    fn test_view_name_mode_default_and_toggle() {
        // OCCT: static bool viewnameMode = false;
        assert!(!XCAFPrs::GetViewNameMode());
        XCAFPrs::SetViewNameMode(true);
        assert!(XCAFPrs::GetViewNameMode());
        XCAFPrs::SetViewNameMode(false);
        assert!(!XCAFPrs::GetViewNameMode());
    }

    #[test]
    fn test_collect_styles_simple_color() {
        let mut label = TDF_Label::new("shape");
        label.color_gen = Some(RED);
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&label, &mut settings, Quantity_ColorRGBA::WHITE);
        let style = settings.get("shape").expect("style bound");
        // ColorGen fills both surface and curve colors (fillStyleColors).
        assert_eq!(style.color_surf, Some(RED));
        assert_eq!(style.color_curv, Some(RED));
        assert!(style.visible);
    }

    #[test]
    fn test_collect_styles_surf_overrides_gen() {
        let mut label = TDF_Label::new("shape");
        label.color_gen = Some(RED);
        label.color_surf = Some(GREEN);
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&label, &mut settings, Quantity_ColorRGBA::WHITE);
        let style = settings.get("shape").unwrap();
        assert_eq!(style.color_surf, Some(GREEN));
        assert_eq!(style.color_curv, Some(RED));
    }

    #[test]
    fn test_collect_styles_empty_style_skipped() {
        // A visible label without any color yields an empty style: not bound.
        let label = TDF_Label::new("shape");
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&label, &mut settings, Quantity_ColorRGBA::WHITE);
        assert!(settings.is_empty());
    }

    #[test]
    fn test_collect_styles_invisible_is_not_empty() {
        // Visibility false makes the style non-empty even without colors.
        let mut label = TDF_Label::new("shape");
        label.visible = false;
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&label, &mut settings, Quantity_ColorRGBA::WHITE);
        let style = settings.get("shape").unwrap();
        assert!(!style.visible);
    }

    #[test]
    fn test_collect_styles_own_color_overrides_referred() {
        // Label with its own ColorGen referring to a shape that also has one:
        // the referred shape is processed first, then the outer label's
        // binding for the same shape id overrides it.
        let mut referred = TDF_Label::new("shape");
        referred.color_gen = Some(RED);
        let mut outer = TDF_Label::new("shape");
        outer.color_gen = Some(GREEN);
        outer.reference = Some(Box::new(referred));
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&outer, &mut settings, Quantity_ColorRGBA::WHITE);
        assert_eq!(settings.get("shape").unwrap().color_surf, Some(GREEN));
    }

    #[test]
    fn test_collect_styles_color_by_layer() {
        let mut label = TDF_Label::new("shape");
        label.is_color_by_layer = true;
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&label, &mut settings, GREEN);
        let style = settings.get("shape").unwrap();
        assert_eq!(style.color_surf, Some(GREEN));
        assert_eq!(style.color_curv, Some(GREEN));
    }

    #[test]
    fn test_collect_styles_components_and_subshapes() {
        let mut comp = TDF_Label::new("comp-shape");
        comp.color_gen = Some(RED);
        let mut sub = TDF_Label::new("sub-shape");
        sub.color_gen = Some(GREEN);
        let mut asm = TDF_Label::new("asm-shape");
        asm.components.push(comp);
        asm.subshapes.push(sub);
        let mut settings = HashMap::new();
        XCAFPrs::CollectStyleSettings(&asm, &mut settings, Quantity_ColorRGBA::WHITE);
        assert_eq!(settings.len(), 2);
        assert_eq!(settings.get("comp-shape").unwrap().color_surf, Some(RED));
        assert_eq!(settings.get("sub-shape").unwrap().color_surf, Some(GREEN));
    }
}
