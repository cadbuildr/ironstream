use ironstream::opengl_shader::*;

#[test]
fn shader_object_compiled() {
    let s = OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main() {}");
    assert!(s.is_compiled());
    assert_eq!(s.stage(), ShaderStageType::Vertex);
}

#[test]
fn shader_object_empty_not_compiled() {
    let s = OpenGlShaderObject::new(1, ShaderStageType::Fragment, "");
    assert!(!s.is_compiled());
}

#[test]
fn program_link_success_vert_frag() {
    let mut p = OpenGlShaderProgram::new(1);
    p.attach_shader(OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main(){}"));
    p.attach_shader(OpenGlShaderObject::new(2, ShaderStageType::Fragment, "void main(){}"));
    assert!(p.link());
    assert!(p.is_linked());
    assert_eq!(p.nb_shaders(), 2);
}

#[test]
fn program_link_fail_no_fragment() {
    let mut p = OpenGlShaderProgram::new(2);
    p.attach_shader(OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main(){}"));
    assert!(!p.link());
    assert!(!p.is_linked());
    assert!(!p.error_log.is_empty());
}

#[test]
fn program_uniforms_set_get_float_and_vec3() {
    let mut p = OpenGlShaderProgram::new(1);
    p.set_uniform_float("uTime", 1.5);
    p.set_uniform_vec3("uColor", [1.0, 0.5, 0.0]);
    assert!(p.has_uniform("uTime"));
    assert!(p.has_uniform("uColor"));
    assert!(!p.has_uniform("uMissing"));
    match p.get_uniform("uTime") {
        Some(UniformValue::Float(v)) => assert!((*v - 1.5).abs() < 1e-6),
        _ => panic!("expected Float uniform"),
    }
    match p.get_uniform("uColor") {
        Some(UniformValue::Vec3(v)) => {
            assert!((v[0] - 1.0).abs() < 1e-6);
            assert!((v[1] - 0.5).abs() < 1e-6);
        }
        _ => panic!("expected Vec3 uniform"),
    }
}
