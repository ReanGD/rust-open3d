#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra as na;

mod tree;
mod mesh;
mod utils;
mod camera;
mod shader;
mod texture;
mod landscape;
mod uniform_filler;

use na::*;

fn main() {
    use glium::{DisplayBuild, Surface};

    let display: glium::backend::glutin_backend::GlutinFacade = glium::glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title(format!("open3d"))
        .build_glium()
        .unwrap();

    let window = display.get_window().unwrap();

    let mut camera = camera::Camera::new(window, Pnt3::new(0.0, 1.2, 0.0), 0.0_f32);
    let model = Mat4::<f32>::new_identity(4);

    let landscape = landscape::Landscape::new(&display).unwrap();
    let tree = tree::Tree::new(&display).unwrap();


    loop {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        {
            camera.update(na::Vec2::new(width, height));

            for ev in display.poll_events() {
                camera.process_input(&ev);
                match ev {
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Escape)) => {
                        target.finish().unwrap();
                        return;
                    },
                    glium::glutin::Event::Closed => {
                        target.finish().unwrap();
                        return;
                    },
                    _ => ()
                }
            }
        }
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // let uniforms_landscape = uniform! {
        //     proj: camera.proj(),
        //     view: camera.view(),
        //     model: model,
        //     tex: &landscape.texture.data
        // };
        let mut uniform = uniform_filler::UniformFiller::new();
        uniform.add(&camera);

        uniform.set_shader(&landscape.shader);
        target.draw(&landscape.model.vb, &landscape.model.ib, &landscape.shader.data, &uniform,
                    &Default::default()).unwrap();
        uniform.set_shader(&tree.shader);
        target.draw(&tree.model.vb, &tree.model.ib, &tree.shader.data, &uniform,
                    &Default::default()).unwrap();

        target.finish().unwrap();
    }
}
