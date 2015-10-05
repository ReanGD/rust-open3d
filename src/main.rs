#[macro_use]
extern crate glium;
extern crate nalgebra as na;

mod camera;
mod landscape;
mod mesh;

use na::*;

fn main() {
    use glium::{DisplayBuild, Surface};

    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("open3d"))
        .build_glium()
        .unwrap();

    let window = display.get_window().unwrap();

    let vertex_shader_src = r#"
    #version 140

    in vec3 position;

    uniform mat4 proj;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        gl_Position = proj * (view * model) * vec4(position, 1.0);
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
    let mut camera = camera::Camera::new(window, Pnt3::new(0.0, 1.2, 0.0), 0.0_f32);
    let model = Mat4::<f32>::new_identity(4);

    let landscape = landscape::Landscape::new(&display).unwrap();

    loop {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        camera.update(na::Vec2::new(width, height));

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            proj: camera.proj(),
            view: camera.view(),
            model: model,
        };

        target.draw(&landscape.model.vb, &landscape.model.ib, &program, &uniforms,
                    &Default::default()).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            camera.process_input(&ev);
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
