use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;

pub struct Shader {
    pub data: Program,
    pub uniform_keys: Vec<String>,
}

impl Shader {
    pub fn new(facade: &GlutinFacade, vertex_shader: &str, fragment_shader: &str) -> Result<Shader, String> {
        let data = match Program::from_source(facade, vertex_shader, fragment_shader, None) {
            Ok(v) => v,
            Err(e) => return Err(format!("Error create shaders: {}", e))
        };

        let mut uniform_keys = Vec::<String>::new();
        for (name, _) in data.uniforms() {
            uniform_keys.push(name.clone());
        }

        Ok(Shader {
            data: data,
            uniform_keys: uniform_keys,
        })
    }
}
