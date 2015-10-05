use glium::{VertexBuffer, IndexBuffer};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{PrimitiveType, BufferCreationError};

#[derive(Copy, Clone)]
pub struct BaseVertex {
    pub position: [f32; 3],
}

implement_vertex!(BaseVertex, position);

pub struct Mesh {
    pub vb: VertexBuffer<BaseVertex>,
    pub ib: IndexBuffer<u32>,
}

impl Mesh {
    pub fn new(facade: &GlutinFacade, vb_data: &[BaseVertex], ib_data: &[u32]) -> Result<Mesh, String>
    {
        let vb = match VertexBuffer::new(facade, vb_data) {
            Ok(vb) => vb,
            Err(e) => return Err(format!("Error create vertex buffer: {}", e))
        };

        let ib = match IndexBuffer::new(facade, PrimitiveType::TrianglesList, ib_data) {
            Ok(ib) => ib,
            Err(e) => return Err(format!("Error create index buffer: {}", match e {
                BufferCreationError::IndexTypeNotSupported => format!("IndexTypeNotSupported"),
                BufferCreationError::PrimitiveTypeNotSupported => format!("PrimitiveTypeNotSupported"),
                BufferCreationError::BufferCreationError(creation_err) => format!("BufferCreationError={}", creation_err),
            }))
        };

        Ok(Mesh {
            vb: vb,
            ib: ib,
        })
    }
}
