use shader::Shader;
use std::collections::HashMap;
use glium::uniforms::{UniformValue, Uniforms};

pub trait Filler {
    fn prefix<'a>(&'a self) -> String;
    fn uniform_value<'a>(&'a self, name: &str) -> Option<UniformValue>;
}

pub struct UniformFiller<'a> {
    fillers: HashMap<String, &'a Filler>,
    shader: Option<&'a Shader>,
}

impl<'a> UniformFiller<'a> {
    pub fn new() -> UniformFiller<'a> {
        UniformFiller {
            fillers: HashMap::<String, &'a Filler>::new(),
            shader: None
        }
    }

    pub fn add<T>(&mut self, filler: &'a T) where T: Filler {
        let _ = self.fillers.entry(filler.prefix()).or_insert(filler);
    }

    pub fn set_shader(&mut self, shader: &'a Shader) {
        self.shader = Some(shader);
    }
}

impl<'a> Uniforms for UniformFiller<'a>  {
    fn visit_values<'c, F: FnMut(&str, UniformValue<'c>)>(&'c self, mut f: F) {
        let uniform_keys: &Vec<String> = &match self.shader {
            Some(ref s) => s,
            None => panic!("Not set shader for Uniformfiller"),
        }.uniform_keys;

        for name in uniform_keys {
            let prefix = match name.find("_") {
                Some(ind) => &name[0..ind],
                None => panic!(format!("Not found '_' in uniform name {} ", name))
            };
            f(&name, match self.fillers[prefix].uniform_value(&name) {
                Some(v) => v,
                None => panic!(format!("Not found uniform name {} in filler", name))
            });
        }
    }
}
