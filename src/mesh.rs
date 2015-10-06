use glium::{Vertex, VertexBuffer, IndexBuffer};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType};

#[derive(Copy, Clone)]
pub struct VertexBase {
    pub pos: [f32; 3],
}

implement_vertex!(VertexBase, pos);

#[derive(Copy, Clone)]
pub struct VertexTex {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
}

implement_vertex!(VertexTex, pos, tex);

pub struct Mesh<TVertex> where TVertex: Copy {
    pub vb: VertexBuffer<TVertex>,
    pub ib: IndexBuffer<u32>,
}

impl<TVertex> Mesh<TVertex> where TVertex: Vertex {
    pub fn new(facade: &GlutinFacade, vb_data: &[TVertex], ib_data: &[u32]) -> Result<Mesh<TVertex>, String>
    {
        let vb = match VertexBuffer::new(facade, vb_data) {
            Ok(vb) => vb,
            Err(e) => return Err(format!("Error create vertex buffer: {}", e))
        };

        let ib = match IndexBuffer::new(facade, PrimitiveType::TrianglesList, ib_data) {
            Ok(ib) => ib,
            Err(e) => return Err(format!("Error create index buffer: {:?}", e))
        };

        Ok(Mesh {
            vb: vb,
            ib: ib,
        })
    }
}
