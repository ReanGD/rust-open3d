use std;
use utils;
use image;
use glium::uniforms::{UniformValue, AsUniformValue};
use glium::texture::texture2d::Texture2d;
use glium::backend::glutin_backend::GlutinFacade;

pub struct Texture {
    pub data: Texture2d,
}

impl Texture {
    pub fn new(facade: &GlutinFacade, filename: &std::path::Path) -> Result<Texture, String> {
        let texture_path = try!(utils::get_base_dir()).join(filename);
        println!("load texture: \"{}\"", texture_path.display());

        let image = match image::open(texture_path.as_path()) {
            Ok(v) => v,
            Err(e) => return Err(format!("error load texture \"{}\": {}", texture_path.display(), e))
        };

        let texture = match Texture2d::new(facade, image) {
            Ok(v) => v,
            Err(e) => return Err(format!("error create texture \"{}\": {:?}", texture_path.display(), e))
        };

        Ok(Texture {
            data: texture,
        })
    }

    pub fn as_uniform_value<'a>(&self) -> UniformValue<'a> {
        let tex: &'a Texture2d = &(self.data);
        tex.as_uniform_value()
    }

}
