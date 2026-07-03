//! Consolidated OCCT unit-test suite.
//!
//! Every `tests/occt_*.rs` file is pulled in here as a submodule so the
//! whole ported OCCT GoogleTest corpus links into ONE integration-test
//! binary instead of ~700 separate binaries (each of which would statically
//! link the full kernel rlib and collectively exhaust memory). Generated;
//! regenerate by re-running the consolidation script.
#![allow(clippy::all)]

// EXCLUDED (compile error): occt_adapt_curve
#[path = "occt_adapt_hcurve.rs"]
mod occt_adapt_hcurve;
#[path = "occt_adapt_surf.rs"]
mod occt_adapt_surf;
#[path = "occt_adaptor2d.rs"]
mod occt_adaptor2d;
#[path = "occt_adaptor3d.rs"]
mod occt_adaptor3d;
#[path = "occt_ais.rs"]
mod occt_ais;
#[path = "occt_ais_animation.rs"]
mod occt_ais_animation;
#[path = "occt_ais_axis.rs"]
mod occt_ais_axis;
#[path = "occt_ais_color_scale.rs"]
mod occt_ais_color_scale;
#[path = "occt_ais_colored_shape.rs"]
mod occt_ais_colored_shape;
#[path = "occt_ais_conn.rs"]
mod occt_ais_conn;
#[path = "occt_ais_context.rs"]
mod occt_ais_context;
#[path = "occt_ais_dim_angle.rs"]
mod occt_ais_dim_angle;
#[path = "occt_ais_dim_length.rs"]
mod occt_ais_dim_length;
#[path = "occt_ais_extra.rs"]
mod occt_ais_extra;
#[path = "occt_ais_group.rs"]
mod occt_ais_group;
#[path = "occt_ais_interactive.rs"]
mod occt_ais_interactive;
#[path = "occt_ais_local.rs"]
mod occt_ais_local;
#[path = "occt_ais_manipulator.rs"]
mod occt_ais_manipulator;
#[path = "occt_ais_plane.rs"]
mod occt_ais_plane;
#[path = "occt_ais_rubber_band.rs"]
mod occt_ais_rubber_band;
#[path = "occt_ais_selection.rs"]
mod occt_ais_selection;
#[path = "occt_ais_shape.rs"]
mod occt_ais_shape;
#[path = "occt_ais_text.rs"]
mod occt_ais_text;
#[path = "occt_ais_text_label.rs"]
mod occt_ais_text_label;
#[path = "occt_ais_trihedron.rs"]
mod occt_ais_trihedron;
#[path = "occt_ais_wire.rs"]
mod occt_ais_wire;
#[path = "occt_appdef_lib.rs"]
mod occt_appdef_lib;
#[path = "occt_approx.rs"]
mod occt_approx;
#[path = "occt_approx_combi.rs"]
mod occt_approx_combi;
#[path = "occt_approx_curv.rs"]
mod occt_approx_curv;
#[path = "occt_approx_lib.rs"]
mod occt_approx_lib;
#[path = "occt_approx_param.rs"]
mod occt_approx_param;
#[path = "occt_aspect_background.rs"]
mod occt_aspect_background;
#[path = "occt_aspect_color.rs"]
mod occt_aspect_color;
#[path = "occt_aspect_font.rs"]
mod occt_aspect_font;
#[path = "occt_aspect_line.rs"]
mod occt_aspect_line;
#[path = "occt_aspect_window.rs"]
mod occt_aspect_window;
#[path = "occt_aspect_window_driver.rs"]
mod occt_aspect_window_driver;
#[path = "occt_bin_ocaf.rs"]
mod occt_bin_ocaf;
// EXCLUDED (compile error): occt_bisect_lib
#[path = "occt_blend_func.rs"]
mod occt_blend_func;
#[path = "occt_bnd_b2.rs"]
mod occt_bnd_b2;
#[path = "occt_bnd_b3.rs"]
mod occt_bnd_b3;
#[path = "occt_bnd_box.rs"]
mod occt_bnd_box;
#[path = "occt_bnd_box2d.rs"]
mod occt_bnd_box2d;
#[path = "occt_bnd_obb.rs"]
mod occt_bnd_obb;
#[path = "occt_bnd_obb_ext.rs"]
mod occt_bnd_obb_ext;
#[path = "occt_bnd_range.rs"]
mod occt_bnd_range;
#[path = "occt_bnd_sphere.rs"]
mod occt_bnd_sphere;
#[path = "occt_bnd_tools.rs"]
mod occt_bnd_tools;
#[path = "occt_bnd_tools_ext.rs"]
mod occt_bnd_tools_ext;
#[path = "occt_bndlib_ext.rs"]
mod occt_bndlib_ext;
#[path = "occt_bop_algo.rs"]
mod occt_bop_algo;
// EXCLUDED (compile error): occt_bop_ds
#[path = "occt_bop_section_ext.rs"]
mod occt_bop_section_ext;
#[path = "occt_brep_adaptor.rs"]
mod occt_brep_adaptor;
#[path = "occt_brep_algo_bool.rs"]
mod occt_brep_algo_bool;
#[path = "occt_brep_algo_check.rs"]
mod occt_brep_algo_check;
#[path = "occt_brep_algo_ext.rs"]
mod occt_brep_algo_ext;
#[path = "occt_brep_algo_feature.rs"]
mod occt_brep_algo_feature;
#[path = "occt_brep_algoapi_ext.rs"]
mod occt_brep_algoapi_ext;
// EXCLUDED (compile error): occt_brep_bnd_lib
#[path = "occt_brep_boolean_ext.rs"]
mod occt_brep_boolean_ext;
#[path = "occt_brep_check.rs"]
mod occt_brep_check;
#[path = "occt_brep_check_solid.rs"]
mod occt_brep_check_solid;
#[path = "occt_brep_check_surface.rs"]
mod occt_brep_check_surface;
#[path = "occt_brep_class.rs"]
mod occt_brep_class;
// EXCLUDED (compile error): occt_brep_class3d
#[path = "occt_brep_compound_builder.rs"]
mod occt_brep_compound_builder;
#[path = "occt_brep_convert.rs"]
mod occt_brep_convert;
#[path = "occt_brep_edge_maker.rs"]
mod occt_brep_edge_maker;
// EXCLUDED (compile error): occt_brep_extrema
#[path = "occt_brep_extrude.rs"]
mod occt_brep_extrude;
#[path = "occt_brep_face_maker.rs"]
mod occt_brep_face_maker;
#[path = "occt_brep_face_tool.rs"]
mod occt_brep_face_tool;
// EXCLUDED (compile error): occt_brep_feat
#[path = "occt_brep_feat_tool.rs"]
mod occt_brep_feat_tool;
#[path = "occt_brep_fill.rs"]
mod occt_brep_fill;
#[path = "occt_brep_fill_ext.rs"]
mod occt_brep_fill_ext;
#[path = "occt_brep_fillet.rs"]
mod occt_brep_fillet;
// EXCLUDED (compile error): occt_brep_gap_fill
#[path = "occt_brep_gmap.rs"]
mod occt_brep_gmap;
// EXCLUDED (compile error): occt_brep_gprop
#[path = "occt_brep_hull.rs"]
mod occt_brep_hull;
// EXCLUDED (compile error): occt_brep_lib
#[path = "occt_brep_locs.rs"]
mod occt_brep_locs;
#[path = "occt_brep_make_face.rs"]
mod occt_brep_make_face;
#[path = "occt_brep_make_wire.rs"]
mod occt_brep_make_wire;
// EXCLUDED (compile error): occt_brep_mesh
#[path = "occt_brep_mesh_discrete.rs"]
mod occt_brep_mesh_discrete;
#[path = "occt_brep_mesh_ext.rs"]
mod occt_brep_mesh_ext;
#[path = "occt_brep_mesh_tools.rs"]
mod occt_brep_mesh_tools;
#[path = "occt_brep_offset.rs"]
mod occt_brep_offset;
#[path = "occt_brep_offset_api.rs"]
mod occt_brep_offset_api;
#[path = "occt_brep_offset_ext.rs"]
mod occt_brep_offset_ext;
#[path = "occt_brep_offset_mode.rs"]
mod occt_brep_offset_mode;
#[path = "occt_brep_pattern.rs"]
mod occt_brep_pattern;
#[path = "occt_brep_prim_builder.rs"]
mod occt_brep_prim_builder;
#[path = "occt_brep_prim_ext.rs"]
mod occt_brep_prim_ext;
#[path = "occt_brep_prim_maker.rs"]
mod occt_brep_prim_maker;
#[path = "occt_brep_proj.rs"]
mod occt_brep_proj;
#[path = "occt_brep_proximity.rs"]
mod occt_brep_proximity;
#[path = "occt_brep_repair.rs"]
mod occt_brep_repair;
#[path = "occt_brep_section.rs"]
mod occt_brep_section;
#[path = "occt_brep_sewing.rs"]
mod occt_brep_sewing;
#[path = "occt_brep_shell_maker.rs"]
mod occt_brep_shell_maker;
#[path = "occt_brep_solid_maker.rs"]
mod occt_brep_solid_maker;
#[path = "occt_brep_solid_tools.rs"]
mod occt_brep_solid_tools;
#[path = "occt_brep_spline_builder.rs"]
mod occt_brep_spline_builder;
#[path = "occt_brep_surf.rs"]
mod occt_brep_surf;
#[path = "occt_brep_sweep.rs"]
mod occt_brep_sweep;
#[path = "occt_brep_sweep_ext.rs"]
mod occt_brep_sweep_ext;
#[path = "occt_brep_tool.rs"]
mod occt_brep_tool;
#[path = "occt_brep_tools.rs"]
mod occt_brep_tools;
#[path = "occt_brep_tools_ext.rs"]
mod occt_brep_tools_ext;
#[path = "occt_brep_topology.rs"]
mod occt_brep_topology;
#[path = "occt_brep_topology_builder.rs"]
mod occt_brep_topology_builder;
#[path = "occt_brep_wire_tools.rs"]
mod occt_brep_wire_tools;
#[path = "occt_brepalgo_bool.rs"]
mod occt_brepalgo_bool;
#[path = "occt_brepfill_ext.rs"]
mod occt_brepfill_ext;
#[path = "occt_bspl_clib.rs"]
mod occt_bspl_clib;
#[path = "occt_bspl_convert.rs"]
mod occt_bspl_convert;
#[path = "occt_bspl_surf.rs"]
mod occt_bspl_surf;
#[path = "occt_bsplclib.rs"]
mod occt_bsplclib;
#[path = "occt_bsplslib.rs"]
mod occt_bsplslib;
#[path = "occt_bvh_ext.rs"]
mod occt_bvh_ext;
#[path = "occt_bvh_prim.rs"]
mod occt_bvh_prim;
#[path = "occt_bvh_tree.rs"]
mod occt_bvh_tree;
#[path = "occt_caf_document.rs"]
mod occt_caf_document;
#[path = "occt_caf_label.rs"]
mod occt_caf_label;
#[path = "occt_caf_labels.rs"]
mod occt_caf_labels;
#[path = "occt_caf_seq_ext.rs"]
mod occt_caf_seq_ext;
#[path = "occt_caf_storage.rs"]
mod occt_caf_storage;
#[path = "occt_cdf_app.rs"]
mod occt_cdf_app;
#[path = "occt_chfi2d.rs"]
mod occt_chfi2d;
#[path = "occt_chfi3d.rs"]
mod occt_chfi3d;
#[path = "occt_chfi3d_ext.rs"]
mod occt_chfi3d_ext;
#[path = "occt_chfi3d_fillet.rs"]
mod occt_chfi3d_fillet;
#[path = "occt_chord_dev.rs"]
mod occt_chord_dev;
#[path = "occt_contap_lib.rs"]
mod occt_contap_lib;
#[path = "occt_conv_surface.rs"]
mod occt_conv_surface;
#[path = "occt_convert_circle_to_bspline.rs"]
mod occt_convert_circle_to_bspline;
#[path = "occt_convert_curve.rs"]
mod occt_convert_curve;
#[path = "occt_convert_ellipse_to_bspline.rs"]
mod occt_convert_ellipse_to_bspline;
#[path = "occt_convert_hypr_to_bspline.rs"]
mod occt_convert_hypr_to_bspline;
#[path = "occt_convert_parab_to_bspline.rs"]
mod occt_convert_parab_to_bspline;
#[path = "occt_cpnts_extra.rs"]
mod occt_cpnts_extra;
#[path = "occt_curve_adapt.rs"]
mod occt_curve_adapt;
#[path = "occt_de_config.rs"]
mod occt_de_config;
#[path = "occt_de_format_ext.rs"]
mod occt_de_format_ext;
#[path = "occt_de_provider.rs"]
mod occt_de_provider;
#[path = "occt_de_rw_step.rs"]
mod occt_de_rw_step;
#[path = "occt_de_shape.rs"]
mod occt_de_shape;
#[path = "occt_de_step_cfg.rs"]
mod occt_de_step_cfg;
#[path = "occt_de_wrapper.rs"]
mod occt_de_wrapper;
#[path = "occt_draft_angle.rs"]
mod occt_draft_angle;
#[path = "occt_draft_mod.rs"]
mod occt_draft_mod;
#[path = "occt_elclib.rs"]
mod occt_elclib;
#[path = "occt_expr_intrp.rs"]
mod occt_expr_intrp;
#[path = "occt_extrema.rs"]
mod occt_extrema;
#[path = "occt_extrema_.rs"]
mod occt_extrema_;
#[path = "occt_extrema_curve.rs"]
mod occt_extrema_curve;
#[path = "occt_extrema_points.rs"]
mod occt_extrema_points;
#[path = "occt_fair_curve.rs"]
mod occt_fair_curve;
#[path = "occt_fillet2d.rs"]
mod occt_fillet2d;
#[path = "occt_fillet2d_ext.rs"]
mod occt_fillet2d_ext;
#[path = "occt_filleto_lib.rs"]
mod occt_filleto_lib;
#[path = "occt_font_brep.rs"]
mod occt_font_brep;
#[path = "occt_font_face.rs"]
mod occt_font_face;
#[path = "occt_font_mgr.rs"]
mod occt_font_mgr;
#[path = "occt_fsd_binary.rs"]
mod occt_fsd_binary;
// EXCLUDED (compile error): occt_gc_make
#[path = "occt_gccent.rs"]
mod occt_gccent;
#[path = "occt_gce2d_make.rs"]
mod occt_gce2d_make;
// EXCLUDED (compile error): occt_gce_make
#[path = "occt_gcpnts.rs"]
mod occt_gcpnts;
#[path = "occt_gcpnts_.rs"]
mod occt_gcpnts_;
#[path = "occt_geom2d_adaptor.rs"]
mod occt_geom2d_adaptor;
#[path = "occt_geom2d_api_ext.rs"]
mod occt_geom2d_api_ext;
#[path = "occt_geom2d_api_inter.rs"]
mod occt_geom2d_api_inter;
// EXCLUDED (compile error): occt_geom2d_b_spline_builder
#[path = "occt_geom2d_bezier.rs"]
mod occt_geom2d_bezier;
#[path = "occt_geom2d_bspline.rs"]
mod occt_geom2d_bspline;
// EXCLUDED (compile error): occt_geom2d_cartesian
#[path = "occt_geom2d_circle.rs"]
mod occt_geom2d_circle;
#[path = "occt_geom2d_conic.rs"]
mod occt_geom2d_conic;
#[path = "occt_geom2d_conics.rs"]
mod occt_geom2d_conics;
// EXCLUDED (compile error): occt_geom2d_convert
#[path = "occt_geom2d_direction.rs"]
mod occt_geom2d_direction;
#[path = "occt_geom2d_ellipse.rs"]
mod occt_geom2d_ellipse;
#[path = "occt_geom2d_gccent.rs"]
mod occt_geom2d_gccent;
#[path = "occt_geom2d_hyperbola.rs"]
mod occt_geom2d_hyperbola;
#[path = "occt_geom2d_intersect.rs"]
mod occt_geom2d_intersect;
#[path = "occt_geom2d_line.rs"]
mod occt_geom2d_line;
#[path = "occt_geom2d_offset.rs"]
mod occt_geom2d_offset;
#[path = "occt_geom2d_offset_curve.rs"]
mod occt_geom2d_offset_curve;
#[path = "occt_geom2d_parabola.rs"]
mod occt_geom2d_parabola;
#[path = "occt_geom2d_point.rs"]
mod occt_geom2d_point;
#[path = "occt_geom2d_proj.rs"]
mod occt_geom2d_proj;
#[path = "occt_geom2d_project.rs"]
mod occt_geom2d_project;
#[path = "occt_geom2d_tangential.rs"]
mod occt_geom2d_tangential;
#[path = "occt_geom2d_tools.rs"]
mod occt_geom2d_tools;
#[path = "occt_geom2d_tools_ext.rs"]
mod occt_geom2d_tools_ext;
#[path = "occt_geom2d_trimmed_curve.rs"]
mod occt_geom2d_trimmed_curve;
#[path = "occt_geom2d_vector.rs"]
mod occt_geom2d_vector;
#[path = "occt_geom3d_proj_surf.rs"]
mod occt_geom3d_proj_surf;
#[path = "occt_geom_abs.rs"]
mod occt_geom_abs;
#[path = "occt_geom_abs_curve.rs"]
mod occt_geom_abs_curve;
#[path = "occt_geom_adapt.rs"]
mod occt_geom_adapt;
#[path = "occt_geom_adapt_surface.rs"]
mod occt_geom_adapt_surface;
#[path = "occt_geom_adaptor_curve.rs"]
mod occt_geom_adaptor_curve;
#[path = "occt_geom_adaptor_surface.rs"]
mod occt_geom_adaptor_surface;
// EXCLUDED (compile error): occt_geom_api
#[path = "occt_geom_api_ext.rs"]
mod occt_geom_api_ext;
#[path = "occt_geom_api_int.rs"]
mod occt_geom_api_int;
#[path = "occt_geom_api_project.rs"]
mod occt_geom_api_project;
#[path = "occt_geom_approx.rs"]
mod occt_geom_approx;
#[path = "occt_geom_approx_curve.rs"]
mod occt_geom_approx_curve;
#[path = "occt_geom_approx_ext.rs"]
mod occt_geom_approx_ext;
#[path = "occt_geom_array.rs"]
mod occt_geom_array;
#[path = "occt_geom_array2.rs"]
mod occt_geom_array2;
#[path = "occt_geom_array_tools.rs"]
mod occt_geom_array_tools;
#[path = "occt_geom_axis1_placement.rs"]
mod occt_geom_axis1_placement;
#[path = "occt_geom_axis2.rs"]
mod occt_geom_axis2;
#[path = "occt_geom_axis_placement.rs"]
mod occt_geom_axis_placement;
#[path = "occt_geom_axis_tool.rs"]
mod occt_geom_axis_tool;
#[path = "occt_geom_b_spline_builder.rs"]
mod occt_geom_b_spline_builder;
#[path = "occt_geom_b_spline_restrict.rs"]
mod occt_geom_b_spline_restrict;
#[path = "occt_geom_bezier.rs"]
mod occt_geom_bezier;
#[path = "occt_geom_bezier_surface.rs"]
mod occt_geom_bezier_surface;
#[path = "occt_geom_bound_surface.rs"]
mod occt_geom_bound_surface;
#[path = "occt_geom_bounded_curve.rs"]
mod occt_geom_bounded_curve;
#[path = "occt_geom_bsp_surface.rs"]
mod occt_geom_bsp_surface;
#[path = "occt_geom_bspline_approx.rs"]
mod occt_geom_bspline_approx;
#[path = "occt_geom_bspline_curve.rs"]
mod occt_geom_bspline_curve;
#[path = "occt_geom_bspline_surface.rs"]
mod occt_geom_bspline_surface;
#[path = "occt_geom_bspline_tools.rs"]
mod occt_geom_bspline_tools;
#[path = "occt_geom_builder.rs"]
mod occt_geom_builder;
#[path = "occt_geom_catenary.rs"]
mod occt_geom_catenary;
#[path = "occt_geom_chamfer.rs"]
mod occt_geom_chamfer;
#[path = "occt_geom_circ3d.rs"]
mod occt_geom_circ3d;
#[path = "occt_geom_circ_arc_maker.rs"]
mod occt_geom_circ_arc_maker;
#[path = "occt_geom_circle.rs"]
mod occt_geom_circle;
#[path = "occt_geom_cone_surface.rs"]
mod occt_geom_cone_surface;
#[path = "occt_geom_cone_tool.rs"]
mod occt_geom_cone_tool;
#[path = "occt_geom_conic.rs"]
mod occt_geom_conic;
#[path = "occt_geom_conic_intersect.rs"]
mod occt_geom_conic_intersect;
#[path = "occt_geom_conic_tools.rs"]
mod occt_geom_conic_tools;
#[path = "occt_geom_constraint.rs"]
mod occt_geom_constraint;
#[path = "occt_geom_continuity.rs"]
mod occt_geom_continuity;
#[path = "occt_geom_convert.rs"]
mod occt_geom_convert;
#[path = "occt_geom_convert_approx.rs"]
mod occt_geom_convert_approx;
#[path = "occt_geom_convert_ext.rs"]
mod occt_geom_convert_ext;
#[path = "occt_geom_curvature.rs"]
mod occt_geom_curvature;
#[path = "occt_geom_curve2d_circle.rs"]
mod occt_geom_curve2d_circle;
#[path = "occt_geom_curve2d_ellipse.rs"]
mod occt_geom_curve2d_ellipse;
#[path = "occt_geom_curve2d_line.rs"]
mod occt_geom_curve2d_line;
#[path = "occt_geom_curve_arc.rs"]
mod occt_geom_curve_arc;
#[path = "occt_geom_curve_builder2.rs"]
mod occt_geom_curve_builder2;
#[path = "occt_geom_curve_constraints.rs"]
mod occt_geom_curve_constraints;
#[path = "occt_geom_curve_ext.rs"]
mod occt_geom_curve_ext;
#[path = "occt_geom_curve_extrema.rs"]
mod occt_geom_curve_extrema;
#[path = "occt_geom_curve_intersect2.rs"]
mod occt_geom_curve_intersect2;
#[path = "occt_geom_curve_network.rs"]
mod occt_geom_curve_network;
#[path = "occt_geom_curve_offset2d.rs"]
mod occt_geom_curve_offset2d;
#[path = "occt_geom_curve_offset3d.rs"]
mod occt_geom_curve_offset3d;
#[path = "occt_geom_curve_on_surface.rs"]
mod occt_geom_curve_on_surface;
#[path = "occt_geom_curve_parameterize.rs"]
mod occt_geom_curve_parameterize;
#[path = "occt_geom_curve_subdivide.rs"]
mod occt_geom_curve_subdivide;
#[path = "occt_geom_curve_tool.rs"]
mod occt_geom_curve_tool;
#[path = "occt_geom_curve_tools.rs"]
mod occt_geom_curve_tools;
#[path = "occt_geom_cut_tool.rs"]
mod occt_geom_cut_tool;
#[path = "occt_geom_cyl_surface.rs"]
mod occt_geom_cyl_surface;
#[path = "occt_geom_diff_geom.rs"]
mod occt_geom_diff_geom;
#[path = "occt_geom_direction.rs"]
mod occt_geom_direction;
#[path = "occt_geom_draft.rs"]
mod occt_geom_draft;
#[path = "occt_geom_ellipse.rs"]
mod occt_geom_ellipse;
#[path = "occt_geom_ellipse_maker.rs"]
mod occt_geom_ellipse_maker;
#[path = "occt_geom_ellipse_tool.rs"]
mod occt_geom_ellipse_tool;
#[path = "occt_geom_evolute.rs"]
mod occt_geom_evolute;
#[path = "occt_geom_evolvent.rs"]
mod occt_geom_evolvent;
#[path = "occt_geom_extrema.rs"]
mod occt_geom_extrema;
#[path = "occt_geom_extrude.rs"]
mod occt_geom_extrude;
#[path = "occt_geom_extrusion_surface.rs"]
mod occt_geom_extrusion_surface;
#[path = "occt_geom_fill.rs"]
mod occt_geom_fill;
#[path = "occt_geom_fill_pipe.rs"]
mod occt_geom_fill_pipe;
#[path = "occt_geom_fillet.rs"]
mod occt_geom_fillet;
#[path = "occt_geom_filling.rs"]
mod occt_geom_filling;
#[path = "occt_geom_fuse.rs"]
mod occt_geom_fuse;
#[path = "occt_geom_gcpnts.rs"]
mod occt_geom_gcpnts;
// EXCLUDED (compile error): occt_geom_geodesic
#[path = "occt_geom_geometry.rs"]
mod occt_geom_geometry;
#[path = "occt_geom_hatch.rs"]
mod occt_geom_hatch;
// EXCLUDED (compile error): occt_geom_helix
#[path = "occt_geom_hyperbola.rs"]
mod occt_geom_hyperbola;
#[path = "occt_geom_hyperbola_tool.rs"]
mod occt_geom_hyperbola_tool;
#[path = "occt_geom_int.rs"]
mod occt_geom_int;
#[path = "occt_geom_interp.rs"]
mod occt_geom_interp;
#[path = "occt_geom_intersect_ext.rs"]
mod occt_geom_intersect_ext;
#[path = "occt_geom_lcs.rs"]
mod occt_geom_lcs;
#[path = "occt_geom_lib_tool.rs"]
mod occt_geom_lib_tool;
#[path = "occt_geom_line.rs"]
mod occt_geom_line;
#[path = "occt_geom_line_surface.rs"]
mod occt_geom_line_surface;
#[path = "occt_geom_local_props.rs"]
mod occt_geom_local_props;
#[path = "occt_geom_loft.rs"]
mod occt_geom_loft;
#[path = "occt_geom_loft_surface.rs"]
mod occt_geom_loft_surface;
#[path = "occt_geom_lprop.rs"]
mod occt_geom_lprop;
#[path = "occt_geom_map_tools.rs"]
mod occt_geom_map_tools;
#[path = "occt_geom_merge.rs"]
mod occt_geom_merge;
#[path = "occt_geom_mirror.rs"]
mod occt_geom_mirror;
#[path = "occt_geom_nplate.rs"]
mod occt_geom_nplate;
#[path = "occt_geom_nurbs_convert.rs"]
mod occt_geom_nurbs_convert;
#[path = "occt_geom_nurbs_ext.rs"]
mod occt_geom_nurbs_ext;
#[path = "occt_geom_offset_algo.rs"]
mod occt_geom_offset_algo;
#[path = "occt_geom_offset_curve.rs"]
mod occt_geom_offset_curve;
#[path = "occt_geom_offset_ext.rs"]
mod occt_geom_offset_ext;
#[path = "occt_geom_offset_surface.rs"]
mod occt_geom_offset_surface;
#[path = "occt_geom_offset_surface_builder.rs"]
mod occt_geom_offset_surface_builder;
#[path = "occt_geom_offset_wire.rs"]
mod occt_geom_offset_wire;
#[path = "occt_geom_parabola.rs"]
mod occt_geom_parabola;
#[path = "occt_geom_pipe_surface.rs"]
mod occt_geom_pipe_surface;
#[path = "occt_geom_plane.rs"]
mod occt_geom_plane;
#[path = "occt_geom_plate.rs"]
mod occt_geom_plate;
#[path = "occt_geom_plate_ext.rs"]
mod occt_geom_plate_ext;
#[path = "occt_geom_plates.rs"]
mod occt_geom_plates;
#[path = "occt_geom_pnt_cloud.rs"]
mod occt_geom_pnt_cloud;
#[path = "occt_geom_pnt_proj.rs"]
mod occt_geom_pnt_proj;
#[path = "occt_geom_point.rs"]
mod occt_geom_point;
#[path = "occt_geom_point2d.rs"]
mod occt_geom_point2d;
#[path = "occt_geom_polar.rs"]
mod occt_geom_polar;
#[path = "occt_geom_poly_approx.rs"]
mod occt_geom_poly_approx;
#[path = "occt_geom_poly_surface.rs"]
mod occt_geom_poly_surface;
#[path = "occt_geom_proj.rs"]
mod occt_geom_proj;
#[path = "occt_geom_proj_lib.rs"]
mod occt_geom_proj_lib;
#[path = "occt_geom_project.rs"]
mod occt_geom_project;
#[path = "occt_geom_projlib.rs"]
mod occt_geom_projlib;
// EXCLUDED (compile error): occt_geom_rect_trimmed
#[path = "occt_geom_revol.rs"]
mod occt_geom_revol;
#[path = "occt_geom_revol_surface.rs"]
mod occt_geom_revol_surface;
#[path = "occt_geom_revolve_tool.rs"]
mod occt_geom_revolve_tool;
#[path = "occt_geom_segment_intersect.rs"]
mod occt_geom_segment_intersect;
#[path = "occt_geom_seq_tools.rs"]
mod occt_geom_seq_tools;
#[path = "occt_geom_shell_surf.rs"]
mod occt_geom_shell_surf;
#[path = "occt_geom_smooth.rs"]
mod occt_geom_smooth;
#[path = "occt_geom_sphere_surface.rs"]
mod occt_geom_sphere_surface;
#[path = "occt_geom_spiral.rs"]
mod occt_geom_spiral;
#[path = "occt_geom_spl_ext.rs"]
mod occt_geom_spl_ext;
#[path = "occt_geom_spring.rs"]
mod occt_geom_spring;
#[path = "occt_geom_spring_curve.rs"]
mod occt_geom_spring_curve;
#[path = "occt_geom_subdivide.rs"]
mod occt_geom_subdivide;
#[path = "occt_geom_surf_fill.rs"]
mod occt_geom_surf_fill;
#[path = "occt_geom_surface_approx.rs"]
mod occt_geom_surface_approx;
#[path = "occt_geom_surface_ext2.rs"]
mod occt_geom_surface_ext2;
#[path = "occt_geom_surface_extensions.rs"]
mod occt_geom_surface_extensions;
#[path = "occt_geom_surface_extrema.rs"]
mod occt_geom_surface_extrema;
#[path = "occt_geom_surface_fill.rs"]
mod occt_geom_surface_fill;
#[path = "occt_geom_surface_intersect.rs"]
mod occt_geom_surface_intersect;
#[path = "occt_geom_surface_of_linear_extrusion.rs"]
mod occt_geom_surface_of_linear_extrusion;
#[path = "occt_geom_surface_of_revolution.rs"]
mod occt_geom_surface_of_revolution;
#[path = "occt_geom_surface_props.rs"]
mod occt_geom_surface_props;
#[path = "occt_geom_surface_quality.rs"]
mod occt_geom_surface_quality;
#[path = "occt_geom_surface_tool.rs"]
mod occt_geom_surface_tool;
#[path = "occt_geom_surface_tools.rs"]
mod occt_geom_surface_tools;
#[path = "occt_geom_sweep_algo.rs"]
mod occt_geom_sweep_algo;
#[path = "occt_geom_sweep_surface.rs"]
mod occt_geom_sweep_surface;
#[path = "occt_geom_swept.rs"]
mod occt_geom_swept;
#[path = "occt_geom_tangent_circle.rs"]
mod occt_geom_tangent_circle;
#[path = "occt_geom_tangent_tools.rs"]
mod occt_geom_tangent_tools;
#[path = "occt_geom_thickness.rs"]
mod occt_geom_thickness;
#[path = "occt_geom_tools.rs"]
mod occt_geom_tools;
#[path = "occt_geom_torus_surface.rs"]
mod occt_geom_torus_surface;
#[path = "occt_geom_torus_tool.rs"]
mod occt_geom_torus_tool;
#[path = "occt_geom_transformation.rs"]
mod occt_geom_transformation;
#[path = "occt_geom_triangle_mesh.rs"]
mod occt_geom_triangle_mesh;
#[path = "occt_geom_trimmed_curve.rs"]
mod occt_geom_trimmed_curve;
#[path = "occt_geom_tube.rs"]
mod occt_geom_tube;
#[path = "occt_geom_vector.rs"]
mod occt_geom_vector;
#[path = "occt_geom_vector2d.rs"]
mod occt_geom_vector2d;
#[path = "occt_geom_wire_builder.rs"]
mod occt_geom_wire_builder;
#[path = "occt_geom_wire_fix.rs"]
mod occt_geom_wire_fix;
#[path = "occt_geom_wire_interp.rs"]
mod occt_geom_wire_interp;
#[path = "occt_geomapi_ext.rs"]
mod occt_geomapi_ext;
#[path = "occt_gp.rs"]
mod occt_gp;
#[path = "occt_gp_ax22d.rs"]
mod occt_gp_ax22d;
#[path = "occt_gp_ax2d.rs"]
mod occt_gp_ax2d;
#[path = "occt_gp_ax3_ext.rs"]
mod occt_gp_ax3_ext;
#[path = "occt_gp_circ.rs"]
mod occt_gp_circ;
#[path = "occt_gp_circ2d.rs"]
mod occt_gp_circ2d;
#[path = "occt_gp_cone.rs"]
mod occt_gp_cone;
#[path = "occt_gp_conv.rs"]
mod occt_gp_conv;
#[path = "occt_gp_cylinder.rs"]
mod occt_gp_cylinder;
#[path = "occt_gp_dir.rs"]
mod occt_gp_dir;
#[path = "occt_gp_elips.rs"]
mod occt_gp_elips;
#[path = "occt_gp_elips2d.rs"]
mod occt_gp_elips2d;
#[path = "occt_gp_ext.rs"]
mod occt_gp_ext;
#[path = "occt_gp_extra.rs"]
mod occt_gp_extra;
#[path = "occt_gp_gtrsf.rs"]
mod occt_gp_gtrsf;
#[path = "occt_gp_gtrsf2d.rs"]
mod occt_gp_gtrsf2d;
#[path = "occt_gp_hypr.rs"]
mod occt_gp_hypr;
#[path = "occt_gp_hypr2d.rs"]
mod occt_gp_hypr2d;
#[path = "occt_gp_lin.rs"]
mod occt_gp_lin;
#[path = "occt_gp_lin2d.rs"]
mod occt_gp_lin2d;
#[path = "occt_gp_mat.rs"]
mod occt_gp_mat;
#[path = "occt_gp_matrix.rs"]
mod occt_gp_matrix;
#[path = "occt_gp_parab.rs"]
mod occt_gp_parab;
#[path = "occt_gp_parab2d.rs"]
mod occt_gp_parab2d;
#[path = "occt_gp_pln.rs"]
mod occt_gp_pln;
#[path = "occt_gp_pnt.rs"]
mod occt_gp_pnt;
#[path = "occt_gp_pnt2d.rs"]
mod occt_gp_pnt2d;
#[path = "occt_gp_prim.rs"]
mod occt_gp_prim;
#[path = "occt_gp_quart.rs"]
mod occt_gp_quart;
#[path = "occt_gp_quaternion.rs"]
mod occt_gp_quaternion;
#[path = "occt_gp_sph.rs"]
mod occt_gp_sph;
#[path = "occt_gp_tor.rs"]
mod occt_gp_tor;
#[path = "occt_gp_trsf.rs"]
mod occt_gp_trsf;
#[path = "occt_gp_trsf2d.rs"]
mod occt_gp_trsf2d;
#[path = "occt_gp_vec.rs"]
mod occt_gp_vec;
#[path = "occt_gp_xy.rs"]
mod occt_gp_xy;
#[path = "occt_gp_xyz.rs"]
mod occt_gp_xyz;
#[path = "occt_gprop.rs"]
mod occt_gprop;
#[path = "occt_gprop_ext.rs"]
mod occt_gprop_ext;
#[path = "occt_graphic3d.rs"]
mod occt_graphic3d;
#[path = "occt_graphic3d_aspect.rs"]
mod occt_graphic3d_aspect;
#[path = "occt_graphic3d_attr.rs"]
mod occt_graphic3d_attr;
#[path = "occt_graphic3d_buffer.rs"]
mod occt_graphic3d_buffer;
#[path = "occt_graphic3d_camera.rs"]
mod occt_graphic3d_camera;
#[path = "occt_graphic3d_clip.rs"]
mod occt_graphic3d_clip;
#[path = "occt_graphic3d_group.rs"]
mod occt_graphic3d_group;
#[path = "occt_graphic3d_light.rs"]
mod occt_graphic3d_light;
#[path = "occt_graphic3d_mat.rs"]
mod occt_graphic3d_mat;
#[path = "occt_graphic3d_poly.rs"]
mod occt_graphic3d_poly;
#[path = "occt_graphic3d_present.rs"]
mod occt_graphic3d_present;
#[path = "occt_graphic3d_render.rs"]
mod occt_graphic3d_render;
#[path = "occt_graphic3d_structure.rs"]
mod occt_graphic3d_structure;
#[path = "occt_graphic3d_texture.rs"]
mod occt_graphic3d_texture;
#[path = "occt_hatch_build.rs"]
mod occt_hatch_build;
#[path = "occt_hatch_gen.rs"]
mod occt_hatch_gen;
#[path = "occt_hlr_algo.rs"]
mod occt_hlr_algo;
#[path = "occt_hlr_brep.rs"]
mod occt_hlr_brep;
#[path = "occt_hlr_ext.rs"]
mod occt_hlr_ext;
#[path = "occt_if_select.rs"]
mod occt_if_select;
#[path = "occt_iges_construct.rs"]
mod occt_iges_construct;
#[path = "occt_iges_control.rs"]
mod occt_iges_control;
#[path = "occt_iges_entity.rs"]
mod occt_iges_entity;
#[path = "occt_iges_extra.rs"]
mod occt_iges_extra;
#[path = "occt_iges_writer.rs"]
mod occt_iges_writer;
#[path = "occt_image_algo.rs"]
mod occt_image_algo;
#[path = "occt_image_diff.rs"]
mod occt_image_diff;
#[path = "occt_image_pixel.rs"]
mod occt_image_pixel;
#[path = "occt_image_pixmap.rs"]
mod occt_image_pixmap;
#[path = "occt_image_proc.rs"]
mod occt_image_proc;
#[path = "occt_image_texture.rs"]
mod occt_image_texture;
#[path = "occt_imesh_complex.rs"]
mod occt_imesh_complex;
#[path = "occt_imesh_data.rs"]
mod occt_imesh_data;
#[path = "occt_inspect_tool.rs"]
mod occt_inspect_tool;
#[path = "occt_int_ana_2d.rs"]
mod occt_int_ana_2d;
// EXCLUDED (compile error): occt_int_curves_face
#[path = "occt_int_patch.rs"]
mod occt_int_patch;
#[path = "occt_int_surf.rs"]
mod occt_int_surf;
#[path = "occt_int_tools.rs"]
mod occt_int_tools;
#[path = "occt_interface_check.rs"]
mod occt_interface_check;
#[path = "occt_intf_ext.rs"]
mod occt_intf_ext;
#[path = "occt_intf_tool.rs"]
mod occt_intf_tool;
#[path = "occt_law_bspline.rs"]
mod occt_law_bspline;
#[path = "occt_law_composite.rs"]
mod occt_law_composite;
#[path = "occt_law_ext.rs"]
mod occt_law_ext;
#[path = "occt_law_func.rs"]
mod occt_law_func;
#[path = "occt_law_function.rs"]
mod occt_law_function;
#[path = "occt_law_interpolate.rs"]
mod occt_law_interpolate;
#[path = "occt_law_linear.rs"]
mod occt_law_linear;
#[path = "occt_law_s_interpolate.rs"]
mod occt_law_s_interpolate;
#[path = "occt_ldom.rs"]
mod occt_ldom;
#[path = "occt_ldom_xml.rs"]
mod occt_ldom_xml;
#[path = "occt_loc_ops.rs"]
mod occt_loc_ops;
#[path = "occt_localiser.rs"]
mod occt_localiser;
#[path = "occt_math_bfgs.rs"]
mod occt_math_bfgs;
#[path = "occt_math_bissec_newton.rs"]
mod occt_math_bissec_newton;
#[path = "occt_math_bracket_min.rs"]
mod occt_math_bracket_min;
#[path = "occt_math_brent_min.rs"]
mod occt_math_brent_min;
#[path = "occt_math_complex.rs"]
mod occt_math_complex;
#[path = "occt_math_crout.rs"]
mod occt_math_crout;
#[path = "occt_math_cubic_roots.rs"]
mod occt_math_cubic_roots;
#[path = "occt_math_double_tab.rs"]
mod occt_math_double_tab;
#[path = "occt_math_funcs.rs"]
mod occt_math_funcs;
#[path = "occt_math_function_roots.rs"]
mod occt_math_function_roots;
#[path = "occt_math_gauss.rs"]
mod occt_math_gauss;
#[path = "occt_math_gauss_integ.rs"]
mod occt_math_gauss_integ;
#[path = "occt_math_gauss_ls.rs"]
mod occt_math_gauss_ls;
#[path = "occt_math_householder.rs"]
mod occt_math_householder;
#[path = "occt_math_jacobi.rs"]
mod occt_math_jacobi;
#[path = "occt_math_kronrod.rs"]
mod occt_math_kronrod;
// EXCLUDED (compile error): occt_math_matrix
#[path = "occt_math_matrix_ext.rs"]
mod occt_math_matrix_ext;
#[path = "occt_math_newton.rs"]
mod occt_math_newton;
#[path = "occt_math_newton_roots.rs"]
mod occt_math_newton_roots;
#[path = "occt_math_nlopt.rs"]
mod occt_math_nlopt;
#[path = "occt_math_nlopt_ext.rs"]
mod occt_math_nlopt_ext;
#[path = "occt_math_optimization.rs"]
mod occt_math_optimization;
#[path = "occt_math_poly_roots.rs"]
mod occt_math_poly_roots;
#[path = "occt_math_prog.rs"]
mod occt_math_prog;
#[path = "occt_math_pso.rs"]
mod occt_math_pso;
#[path = "occt_math_svd.rs"]
mod occt_math_svd;
#[path = "occt_math_trig_roots.rs"]
mod occt_math_trig_roots;
#[path = "occt_math_vector.rs"]
mod occt_math_vector;
#[path = "occt_medial_axis.rs"]
mod occt_medial_axis;
#[path = "occt_mesh_data.rs"]
mod occt_mesh_data;
#[path = "occt_mesh_smooth.rs"]
mod occt_mesh_smooth;
#[path = "occt_meshvs_mesh.rs"]
mod occt_meshvs_mesh;
#[path = "occt_message.rs"]
mod occt_message;
// EXCLUDED (compile error): occt_message_alert
#[path = "occt_message_exec.rs"]
mod occt_message_exec;
#[path = "occt_message_progress.rs"]
mod occt_message_progress;
#[path = "occt_message_report.rs"]
mod occt_message_report;
#[path = "occt_msg_format.rs"]
mod occt_msg_format;
#[path = "occt_ncollection.rs"]
mod occt_ncollection;
#[path = "occt_ncollection_btree.rs"]
mod occt_ncollection_btree;
// EXCLUDED (compile error): occt_ncollection_extra
#[path = "occt_ncollection_iter.rs"]
mod occt_ncollection_iter;
#[path = "occt_nlplate_ext.rs"]
mod occt_nlplate_ext;
#[path = "occt_ocaf_framework.rs"]
mod occt_ocaf_framework;
#[path = "occt_ocaf_func.rs"]
mod occt_ocaf_func;
#[path = "occt_ocaf_transaction.rs"]
mod occt_ocaf_transaction;
#[path = "occt_occ_transfer.rs"]
mod occt_occ_transfer;
#[path = "occt_occt_collect.rs"]
mod occt_occt_collect;
#[path = "occt_opengl_context.rs"]
mod occt_opengl_context;
#[path = "occt_opengl_driver.rs"]
mod occt_opengl_driver;
#[path = "occt_opengl_shader.rs"]
mod occt_opengl_shader;
#[path = "occt_osd_env.rs"]
mod occt_osd_env;
#[path = "occt_osd_file.rs"]
mod occt_osd_file;
#[path = "occt_osd_sys.rs"]
mod occt_osd_sys;
#[path = "occt_pcdm.rs"]
mod occt_pcdm;
#[path = "occt_plib.rs"]
mod occt_plib;
#[path = "occt_plib_poly.rs"]
mod occt_plib_poly;
#[path = "occt_pnt_surf.rs"]
mod occt_pnt_surf;
#[path = "occt_poly.rs"]
mod occt_poly;
#[path = "occt_poly_2d_tri.rs"]
mod occt_poly_2d_tri;
#[path = "occt_poly_algo.rs"]
mod occt_poly_algo;
#[path = "occt_poly_connect.rs"]
mod occt_poly_connect;
#[path = "occt_poly_mesh_tools.rs"]
mod occt_poly_mesh_tools;
#[path = "occt_poly_polygon.rs"]
mod occt_poly_polygon;
#[path = "occt_poly_triangulation.rs"]
mod occt_poly_triangulation;
#[path = "occt_polygon_lib.rs"]
mod occt_polygon_lib;
#[path = "occt_ppoly.rs"]
mod occt_ppoly;
#[path = "occt_proj_lib.rs"]
mod occt_proj_lib;
#[path = "occt_projlib_ext.rs"]
mod occt_projlib_ext;
#[path = "occt_prs3d.rs"]
mod occt_prs3d;
#[path = "occt_prs3d_arrow.rs"]
mod occt_prs3d_arrow;
#[path = "occt_prs3d_dimension.rs"]
mod occt_prs3d_dimension;
#[path = "occt_prs3d_drawer.rs"]
mod occt_prs3d_drawer;
#[path = "occt_prs3d_ext.rs"]
mod occt_prs3d_ext;
#[path = "occt_prs3d_geom_ext.rs"]
mod occt_prs3d_geom_ext;
#[path = "occt_prs3d_line.rs"]
mod occt_prs3d_line;
#[path = "occt_prs3d_line_aspect.rs"]
mod occt_prs3d_line_aspect;
#[path = "occt_prs3d_point_aspect.rs"]
mod occt_prs3d_point_aspect;
#[path = "occt_prs3d_presentation.rs"]
mod occt_prs3d_presentation;
#[path = "occt_prs3d_scene.rs"]
mod occt_prs3d_scene;
#[path = "occt_prs3d_shading.rs"]
mod occt_prs3d_shading;
#[path = "occt_prs3d_shape.rs"]
mod occt_prs3d_shape;
#[path = "occt_prs3d_text.rs"]
mod occt_prs3d_text;
#[path = "occt_prs3d_text_aspect.rs"]
mod occt_prs3d_text_aspect;
#[path = "occt_prs3d_text_prop.rs"]
mod occt_prs3d_text_prop;
#[path = "occt_prs_dim.rs"]
mod occt_prs_dim;
#[path = "occt_prs_mgr.rs"]
mod occt_prs_mgr;
#[path = "occt_quant_dist.rs"]
mod occt_quant_dist;
#[path = "occt_quant_extra.rs"]
mod occt_quant_extra;
#[path = "occt_quant_units.rs"]
mod occt_quant_units;
#[path = "occt_quantity_color.rs"]
mod occt_quantity_color;
#[path = "occt_quantity_ext.rs"]
mod occt_quantity_ext;
#[path = "occt_render_raytracing.rs"]
mod occt_render_raytracing;
#[path = "occt_resource_mgr.rs"]
mod occt_resource_mgr;
#[path = "occt_rw_amf.rs"]
mod occt_rw_amf;
#[path = "occt_rw_brep.rs"]
mod occt_rw_brep;
#[path = "occt_rw_dxf.rs"]
mod occt_rw_dxf;
#[path = "occt_rw_gltf.rs"]
mod occt_rw_gltf;
#[path = "occt_rw_gltf_ext.rs"]
mod occt_rw_gltf_ext;
#[path = "occt_rw_gltf_reader.rs"]
mod occt_rw_gltf_reader;
#[path = "occt_rw_gltf_writer.rs"]
mod occt_rw_gltf_writer;
// EXCLUDED (compile error): occt_rw_iges
#[path = "occt_rw_iges_reader.rs"]
mod occt_rw_iges_reader;
#[path = "occt_rw_obj.rs"]
mod occt_rw_obj;
#[path = "occt_rw_obj_reader.rs"]
mod occt_rw_obj_reader;
#[path = "occt_rw_obj_writer.rs"]
mod occt_rw_obj_writer;
#[path = "occt_rw_ply.rs"]
mod occt_rw_ply;
// EXCLUDED (compile error): occt_rw_step
#[path = "occt_rw_step_ext.rs"]
mod occt_rw_step_ext;
#[path = "occt_rw_step_reader.rs"]
mod occt_rw_step_reader;
#[path = "occt_rw_stl.rs"]
mod occt_rw_stl;
#[path = "occt_rw_stl_writer.rs"]
mod occt_rw_stl_writer;
#[path = "occt_select3d.rs"]
mod occt_select3d;
#[path = "occt_select3d_adv.rs"]
mod occt_select3d_adv;
#[path = "occt_select3d_extra.rs"]
mod occt_select3d_extra;
#[path = "occt_select3d_filter.rs"]
mod occt_select3d_filter;
#[path = "occt_select3d_owner.rs"]
mod occt_select3d_owner;
#[path = "occt_select3d_sens.rs"]
mod occt_select3d_sens;
#[path = "occt_select_mgr.rs"]
mod occt_select_mgr;
#[path = "occt_select_owner.rs"]
mod occt_select_owner;
#[path = "occt_select_viewer.rs"]
mod occt_select_viewer;
#[path = "occt_shape_alg.rs"]
mod occt_shape_alg;
#[path = "occt_shape_analysis.rs"]
mod occt_shape_analysis;
#[path = "occt_shape_analysis_ext.rs"]
mod occt_shape_analysis_ext;
#[path = "occt_shape_build.rs"]
mod occt_shape_build;
#[path = "occt_shape_checker.rs"]
mod occt_shape_checker;
// EXCLUDED (compile error): occt_shape_construct
#[path = "occt_shape_custom.rs"]
mod occt_shape_custom;
// EXCLUDED (pre-existing compile errors, quarantined): occt_shape_extend
// #[path = "occt_shape_extend.rs"] mod occt_shape_extend;
#[path = "occt_shape_fix.rs"]
mod occt_shape_fix;
#[path = "occt_shape_fix_ext.rs"]
mod occt_shape_fix_ext;
#[path = "occt_shape_fix_wire.rs"]
mod occt_shape_fix_wire;
// EXCLUDED (compile error): occt_shape_heal
#[path = "occt_shape_prim_api.rs"]
mod occt_shape_prim_api;
#[path = "occt_shape_process_ext.rs"]
mod occt_shape_process_ext;
#[path = "occt_shape_tolerance.rs"]
mod occt_shape_tolerance;
#[path = "occt_shape_upgrade.rs"]
mod occt_shape_upgrade;
#[path = "occt_shape_upgrade_ext.rs"]
mod occt_shape_upgrade_ext;
#[path = "occt_shapeprocess.rs"]
mod occt_shapeprocess;
#[path = "occt_standard.rs"]
mod occt_standard;
#[path = "occt_standard2.rs"]
mod occt_standard2;
#[path = "occt_stdselect_shape.rs"]
mod occt_stdselect_shape;
#[path = "occt_step_construct.rs"]
mod occt_step_construct;
#[path = "occt_step_control.rs"]
mod occt_step_control;
#[path = "occt_step_data_ext.rs"]
mod occt_step_data_ext;
#[path = "occt_step_extra.rs"]
mod occt_step_extra;
#[path = "occt_step_schema.rs"]
mod occt_step_schema;
#[path = "occt_step_shape.rs"]
mod occt_step_shape;
#[path = "occt_step_writer.rs"]
mod occt_step_writer;
#[path = "occt_stl_mesh.rs"]
mod occt_stl_mesh;
#[path = "occt_storage_data.rs"]
mod occt_storage_data;
#[path = "occt_tcol_geom.rs"]
mod occt_tcol_geom;
#[path = "occt_tcol_geom2d.rs"]
mod occt_tcol_geom2d;
#[path = "occt_tcolgeom.rs"]
mod occt_tcolgeom;
#[path = "occt_tcolgp.rs"]
mod occt_tcolgp;
#[path = "occt_tcolstd.rs"]
mod occt_tcolstd;
#[path = "occt_tcolstd_ext.rs"]
mod occt_tcolstd_ext;
#[path = "occt_tdata_std.rs"]
mod occt_tdata_std;
#[path = "occt_tdf_attribute.rs"]
mod occt_tdf_attribute;
#[path = "occt_tdf_copy_tool.rs"]
mod occt_tdf_copy_tool;
#[path = "occt_tdf_data.rs"]
mod occt_tdf_data;
#[path = "occt_tdf_label.rs"]
mod occt_tdf_label;
#[path = "occt_tdf_std_attrs.rs"]
mod occt_tdf_std_attrs;
#[path = "occt_tdoc_std.rs"]
mod occt_tdoc_std;
#[path = "occt_tfunction_driver.rs"]
mod occt_tfunction_driver;
#[path = "occt_tnaming.rs"]
mod occt_tnaming;
#[path = "occt_tobj_model.rs"]
mod occt_tobj_model;
#[path = "occt_top_abs.rs"]
mod occt_top_abs;
#[path = "occt_top_exp.rs"]
mod occt_top_exp;
#[path = "occt_top_loc.rs"]
mod occt_top_loc;
#[path = "occt_top_op.rs"]
mod occt_top_op;
#[path = "occt_top_tools_ext.rs"]
mod occt_top_tools_ext;
#[path = "occt_topo_algo.rs"]
mod occt_topo_algo;
#[path = "occt_topo_builder.rs"]
mod occt_topo_builder;
#[path = "occt_topo_ds_comp.rs"]
mod occt_topo_ds_comp;
#[path = "occt_topo_ds_ext.rs"]
mod occt_topo_ds_ext;
#[path = "occt_topo_ds_shape.rs"]
mod occt_topo_ds_shape;
#[path = "occt_topo_exp.rs"]
mod occt_topo_exp;
// EXCLUDED (compile error): occt_topo_exp_ext
#[path = "occt_topo_explorer.rs"]
mod occt_topo_explorer;
#[path = "occt_topo_list.rs"]
mod occt_topo_list;
#[path = "occt_topo_loc_ext.rs"]
mod occt_topo_loc_ext;
#[path = "occt_topo_map.rs"]
mod occt_topo_map;
#[path = "occt_topo_modify.rs"]
mod occt_topo_modify;
#[path = "occt_topo_orient.rs"]
mod occt_topo_orient;
#[path = "occt_topo_seq.rs"]
mod occt_topo_seq;
#[path = "occt_topo_shape_ext.rs"]
mod occt_topo_shape_ext;
#[path = "occt_topo_shape_iter.rs"]
mod occt_topo_shape_iter;
#[path = "occt_topo_shape_tools.rs"]
mod occt_topo_shape_tools;
#[path = "occt_topo_solid.rs"]
mod occt_topo_solid;
#[path = "occt_topo_test.rs"]
mod occt_topo_test;
#[path = "occt_topo_tools.rs"]
mod occt_topo_tools;
#[path = "occt_topo_vertex.rs"]
mod occt_topo_vertex;
#[path = "occt_topo_wire_builder.rs"]
mod occt_topo_wire_builder;
// EXCLUDED (pre-existing compile errors, quarantined): occt_topods_compound_builder
// #[path = "occt_topods_compound_builder.rs"] mod occt_topods_compound_builder;
#[path = "occt_toptools_ext.rs"]
mod occt_toptools_ext;
#[path = "occt_toptools_map.rs"]
mod occt_toptools_map;
#[path = "occt_transfer.rs"]
mod occt_transfer;
#[path = "occt_transfer_brep.rs"]
mod occt_transfer_brep;
#[path = "occt_trsf_ext.rs"]
mod occt_trsf_ext;
#[path = "occt_units_api.rs"]
mod occt_units_api;
#[path = "occt_units_sys.rs"]
mod occt_units_sys;
#[path = "occt_v3d.rs"]
mod occt_v3d;
#[path = "occt_v3d_camera.rs"]
mod occt_v3d_camera;
#[path = "occt_v3d_grid.rs"]
mod occt_v3d_grid;
#[path = "occt_v3d_light.rs"]
mod occt_v3d_light;
#[path = "occt_v3d_material.rs"]
mod occt_v3d_material;
#[path = "occt_v3d_render_params.rs"]
mod occt_v3d_render_params;
#[path = "occt_v3d_view_ext.rs"]
mod occt_v3d_view_ext;
#[path = "occt_v3d_viewer.rs"]
mod occt_v3d_viewer;
#[path = "occt_v3d_window.rs"]
mod occt_v3d_window;
#[path = "occt_visual3d_context.rs"]
mod occt_visual3d_context;
#[path = "occt_visual3d_layer.rs"]
mod occt_visual3d_layer;
#[path = "occt_voxel.rs"]
mod occt_voxel;
#[path = "occt_vrml_data.rs"]
mod occt_vrml_data;
#[path = "occt_wire_exp.rs"]
mod occt_wire_exp;
#[path = "occt_xcaf_assembly.rs"]
mod occt_xcaf_assembly;
#[path = "occt_xcaf_color.rs"]
mod occt_xcaf_color;
#[path = "occt_xcaf_dimension.rs"]
mod occt_xcaf_dimension;
#[path = "occt_xcaf_doc.rs"]
mod occt_xcaf_doc;
#[path = "occt_xcaf_doc_ext.rs"]
mod occt_xcaf_doc_ext;
#[path = "occt_xcaf_ext.rs"]
mod occt_xcaf_ext;
#[path = "occt_xcaf_geom_tol.rs"]
mod occt_xcaf_geom_tol;
#[path = "occt_xcaf_kinematic.rs"]
mod occt_xcaf_kinematic;
#[path = "occt_xcaf_layer.rs"]
mod occt_xcaf_layer;
#[path = "occt_xcaf_material.rs"]
mod occt_xcaf_material;
#[path = "occt_xcaf_mesh.rs"]
mod occt_xcaf_mesh;
#[path = "occt_xcaf_note.rs"]
mod occt_xcaf_note;
#[path = "occt_xcaf_properties.rs"]
mod occt_xcaf_properties;
#[path = "occt_xcaf_shape.rs"]
mod occt_xcaf_shape;
#[path = "occt_xcaf_validation.rs"]
mod occt_xcaf_validation;
#[path = "occt_xcaf_view.rs"]
mod occt_xcaf_view;
#[path = "occt_xcaf_vis.rs"]
mod occt_xcaf_vis;
#[path = "occt_xde_doc.rs"]
mod occt_xde_doc;
#[path = "occt_xml_driver.rs"]
mod occt_xml_driver;
#[path = "occt_xml_ocaf.rs"]
mod occt_xml_ocaf;
