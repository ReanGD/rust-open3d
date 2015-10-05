use glium::backend::glutin_backend::GlutinFacade;
use mesh::{BaseVertex, Mesh};

pub struct Landscape {
    pub model: Mesh,
}

impl Landscape {
    pub fn new(facade: &GlutinFacade) -> Result<Landscape, String>
    {
        let scale = 10.0_f32;
        let vertex0 = BaseVertex { position: [-scale, 0.0, -scale] };
        let vertex1 = BaseVertex { position: [ scale, 0.0, -scale] };
        let vertex2 = BaseVertex { position: [ scale, 0.0,  scale] };
        let vertex3 = BaseVertex { position: [-scale, 0.0,  scale] };

        let mesh = try!(Mesh::new(facade,
                                  &[vertex0, vertex1, vertex2, vertex3],
                                  &[0, 1, 2, 2, 3, 0]));

        Ok(Landscape {
            model: mesh,
        })
    }

}
