use std;
use shader;
use glium::backend::glutin_backend::GlutinFacade;
use mesh::{VertexBase, Mesh};

pub struct Tree {
    pub model: Mesh<VertexBase>,
    pub shader: shader::Shader,
}

impl Tree {
    fn load_shaders(facade: &GlutinFacade) -> Result<shader::Shader, String> {
        let vertex_shader_src = r#"
    #version 140

    in vec3 pos;

    uniform mat4 proj;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        gl_Position = proj * (view * model) * vec4(pos, 1.0);
    }
"#;

        let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 1.0, 0.0, 1.0);
    }
"#;
        Ok(try!(shader::Shader::new(facade, vertex_shader_src, fragment_shader_src)))
    }

    pub fn new(facade: &GlutinFacade) -> Result<Tree, String>
    {
        let height = 2.0_f32;
        let radius = 0.5_f32;
        let cnt_vertex = 10;

        let mut vb = vec![VertexBase { pos: [0.0, height, 0.0] }];
        let mut ib = Vec::<u32>::new();

        let mut angle = 0.0_f32;
        let step = std::f32::consts::PI * 2.0_f32 / (cnt_vertex as f32);
        for i in 1 .. cnt_vertex + 1 {
            let sin = angle.sin();
            let cos = angle.cos();
            vb.push(VertexBase { pos: [radius * cos, 0.0_f32, radius * sin] });
            ib.push(i);
            ib.push(0);
            ib.push(if i == cnt_vertex {1} else {i + 1});

            angle += step;
		}

        let mesh = try!(Mesh::new(facade, &vb, &ib));
        let shader = try!(Tree::load_shaders(facade));

        Ok(Tree {
            model: mesh,
            shader: shader,
        })
    }

}
