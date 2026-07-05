// FILE: open_gl_gl_core15.rs
// occt: OpenGl_GlCore15

/// OpenGL 1.5 core based on 1.4 version.
/// Extends OpenGL 1.4 with buffer objects and query support.
pub struct OpenGlGlCore15;

impl OpenGlGlCore15 {
    /// OpenGL 1.5 (not in ES 2.0): glBeginQuery
    /// Begins recording samples for an occlusion or timer query.
    pub fn gl_begin_query() -> &'static str {
        "glBeginQuery"
    }

    /// OpenGL 1.5 (not in ES 2.0): glDeleteQueries
    /// Deletes query objects.
    pub fn gl_delete_queries() -> &'static str {
        "glDeleteQueries"
    }

    /// OpenGL 1.5 (not in ES 2.0): glEndQuery
    /// Ends recording samples for a query.
    pub fn gl_end_query() -> &'static str {
        "glEndQuery"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGenQueries
    /// Generates query object names.
    pub fn gl_gen_queries() -> &'static str {
        "glGenQueries"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGetQueryiv
    /// Retrieves query state parameters.
    pub fn gl_get_queryiv() -> &'static str {
        "glGetQueryiv"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGetQueryObjectiv
    /// Retrieves integer query results.
    pub fn gl_get_query_objectiv() -> &'static str {
        "glGetQueryObjectiv"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGetQueryObjectuiv
    /// Retrieves unsigned integer query results.
    pub fn gl_get_query_objectuiv() -> &'static str {
        "glGetQueryObjectuiv"
    }

    /// OpenGL 1.5 (not in ES 2.0): glIsQuery
    /// Tests if a query object name is valid.
    pub fn gl_is_query() -> &'static str {
        "glIsQuery"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGetBufferPointerv
    /// Returns the address of a mapped buffer.
    pub fn gl_get_buffer_pointerv() -> &'static str {
        "glGetBufferPointerv"
    }

    /// OpenGL 1.5 (not in ES 2.0): glGetBufferSubData
    /// Retrieves a subset of a buffer object's data store.
    pub fn gl_get_buffer_sub_data() -> &'static str {
        "glGetBufferSubData"
    }

    /// OpenGL 1.5 (not in ES 2.0): glMapBuffer
    /// Maps a buffer object to client memory.
    pub fn gl_map_buffer() -> &'static str {
        "glMapBuffer"
    }

    /// OpenGL 1.5 (not in ES 2.0): glUnmapBuffer
    /// Unmaps a previously mapped buffer object.
    pub fn gl_unmap_buffer() -> &'static str {
        "glUnmapBuffer"
    }

    /// OpenGL 1.5: glBindBuffer
    /// Binds a buffer object to a buffer target.
    pub fn gl_bind_buffer() -> &'static str {
        "glBindBuffer"
    }

    /// OpenGL 1.5: glBufferData
    /// Creates and initializes a buffer object's data store.
    pub fn gl_buffer_data() -> &'static str {
        "glBufferData"
    }

    /// OpenGL 1.5: glBufferSubData
    /// Updates a subset of a buffer object's data store.
    pub fn gl_buffer_sub_data() -> &'static str {
        "glBufferSubData"
    }

    /// OpenGL 1.5: glDeleteBuffers
    /// Deletes buffer objects.
    pub fn gl_delete_buffers() -> &'static str {
        "glDeleteBuffers"
    }

    /// OpenGL 1.5: glGenBuffers
    /// Generates buffer object names.
    pub fn gl_gen_buffers() -> &'static str {
        "glGenBuffers"
    }

    /// OpenGL 1.5: glGetBufferParameteriv
    /// Retrieves buffer object parameter values.
    pub fn gl_get_buffer_parameteriv() -> &'static str {
        "glGetBufferParameteriv"
    }

    /// OpenGL 1.5: glIsBuffer
    /// Tests if a buffer object name is valid.
    pub fn gl_is_buffer() -> &'static str {
        "glIsBuffer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core15_buffer_functions() {
        // Verify OpenGL 1.5 buffer operations
        assert_eq!(OpenGlGlCore15::gl_bind_buffer(), "glBindBuffer");
        assert_eq!(OpenGlGlCore15::gl_buffer_data(), "glBufferData");
        assert_eq!(OpenGlGlCore15::gl_buffer_sub_data(), "glBufferSubData");
        assert_eq!(OpenGlGlCore15::gl_delete_buffers(), "glDeleteBuffers");
        assert_eq!(OpenGlGlCore15::gl_gen_buffers(), "glGenBuffers");
        assert_eq!(OpenGlGlCore15::gl_is_buffer(), "glIsBuffer");
    }

    #[test]
    fn test_gl_core15_query_functions() {
        // Verify OpenGL 1.5 query operations (not in ES 2.0)
        let funcs = vec![
            OpenGlGlCore15::gl_begin_query(),
            OpenGlGlCore15::gl_delete_queries(),
            OpenGlGlCore15::gl_end_query(),
            OpenGlGlCore15::gl_gen_queries(),
            OpenGlGlCore15::gl_get_queryiv(),
        ];

        for func in funcs {
            // OCCT OpenGl_GlCore15 declares glDeleteQueries/glGenQueries with
            // plural "Queries", so match the common stem "Quer".
            assert!(func.contains("Quer"), "Function {} should contain 'Quer'", func);
        }
    }

    #[test]
    fn test_gl_core15_non_es20_buffer_functions() {
        // Verify non-ES 2.0 buffer mapping functions
        let funcs = vec![
            OpenGlGlCore15::gl_get_buffer_pointerv(),
            OpenGlGlCore15::gl_get_buffer_sub_data(),
            OpenGlGlCore15::gl_map_buffer(),
            OpenGlGlCore15::gl_unmap_buffer(),
        ];

        for func in funcs {
            assert!(!func.is_empty());
        }
    }

    #[test]
    fn test_gl_core15_buffer_object_management() {
        // Verify buffer object lifecycle functions exist
        assert!(!OpenGlGlCore15::gl_gen_buffers().is_empty());
        assert!(!OpenGlGlCore15::gl_bind_buffer().is_empty());
        assert!(!OpenGlGlCore15::gl_delete_buffers().is_empty());
    }
}
