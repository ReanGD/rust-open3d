use std;
use glium::backend::glutin_backend::GlutinFacade;
use mesh::{VertexTex, Mesh};
use texture;
use shader;

pub struct Landscape {
    pub model: Mesh<VertexTex>,
    pub shader: shader::Shader,
    pub texture: texture::Texture,
}

impl Landscape {
    fn load_shaders(facade: &GlutinFacade) -> Result<shader::Shader, String> {
        let vertex_shader_src = r#"
    #version 140

    in vec3 pos;
    in vec2 tex;

    out vec2 f_tex;

    uniform mat4 proj;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        f_tex = tex;
        gl_Position = proj * (view * model) * vec4(pos, 1.0);
    }
"#;

        let fragment_shader_src = r#"
    #version 140

    in vec2 f_tex;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, f_tex);
    }
"#;
        Ok(try!(shader::Shader::new(facade, vertex_shader_src, fragment_shader_src)))
    }

    pub fn new(facade: &GlutinFacade) -> Result<Landscape, String>
    {
        let scale = 100.0_f32;
        let tex_scale = 10.0_f32;
        let vertex0 = VertexTex { pos: [-scale, 0.0, -scale], tex: [0.0,       0.0] };
        let vertex1 = VertexTex { pos: [ scale, 0.0, -scale], tex: [tex_scale, 0.0] };
        let vertex2 = VertexTex { pos: [ scale, 0.0,  scale], tex: [tex_scale, tex_scale] };
        let vertex3 = VertexTex { pos: [-scale, 0.0,  scale], tex: [0.0,       tex_scale] };

        let mesh = try!(Mesh::new(facade,
                                  &[vertex0, vertex1, vertex2, vertex3],
                                  &[0, 1, 2, 2, 3, 0]));
        let shader = try!(Landscape::load_shaders(facade));
        let texture = try!(texture::Texture::new(facade, &std::path::Path::new("terrarian.jpg")));

        Ok(Landscape {
            model: mesh,
            shader: shader,
            texture: texture,
        })
    }

}
