#[macro_use]
extern crate glium;
extern crate nalgebra as na;

use na::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("open3d"))
        .build_glium()
        .unwrap();

    let vertex1 = Vertex { position: [-5.0, 0.0, -5.0] };
    let vertex2 = Vertex { position: [ 5.0, 0.0, -5.0] };
    let vertex3 = Vertex { position: [ 5.0, 0.0,  5.0] };
    let vertex4 = Vertex { position: [ 5.0, 0.0,  5.0] };
    let vertex5 = Vertex { position: [-5.0, 0.0,  5.0] };
    let vertex6 = Vertex { position: [-5.0, 0.0, -5.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec3 position;

    uniform mat4 proj;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        gl_Position = proj * view * model * vec4(position, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.0, 1.0, 0.0, 1.0);
    }
"#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    // let mut rot = Rot3::new(Vec3::new(0.0, 0.0, 0.0));

    // let mut t: f32 = -0.5;
    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let proj = {
            let (width, height) = target.get_dimensions();
            let aspect = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
            Persp3::new(aspect, fov, zfar, znear).to_mat()
        };


        let view = {
            let mut camera: Iso3<f32> = one();
            let eye = Pnt3::new(0.0, 0.2, 0.0);
            let at = Pnt3::new(0.5, 0.2, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            camera.look_at(&eye, &at, &up);
            camera.to_homogeneous()
        };

        // let view = view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
                ];

        let uniforms = uniform! {
            proj: *proj.as_array(),
            view: *view.as_array(),
            model: model,
        };

        // t += 0.02;
        // rot.set_rotation(Vec3::new(0.0, t, 0.2));
        // let m: Mat4<f32> = rot.to_homogeneous();

        // let uniforms = uniform! {
        //     matrix: *m.as_array(),
        // };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();

        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
