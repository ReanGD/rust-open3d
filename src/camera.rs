use std::f32;
use glium;
use glium::glutin::{Event, ElementState, VirtualKeyCode};
use na::{self, ToHomogeneous, Inv, Col, Norm, Eye};


pub struct Camera<'a> {
    proj: na::Persp3<f32>,
    rotate: na::Rot3<f32>,
    position: na::Vec4<f32>,
    roll: f32,
    pitch: f32, // over y
    move_forward: i8,
    move_left: i8,
    window: glium::backend::glutin_backend::WinRef<'a>,
    window_size: na::Vec2<u32>,
    window_center: na::Pnt2<i32>,
}

impl<'a> Camera<'a> {
    pub fn new(window: glium::backend::glutin_backend::WinRef<'a>, position: na::Pnt3<f32>, pitch_angle: f32) -> Camera {
        let aspect = 1.0_f32;
        let fov = 100.0_f32 * f32::consts::PI / 180.0_f32;
        let zfar = 1000.0_f32;
        let znear = 0.1_f32;
        let proj = na::Persp3::new(aspect, fov, zfar, znear);

        Camera {
            proj: proj,
            rotate: na::Rot3::new_identity(3),
            position: na::Vec4::new(position.x, position.y, position.z, 1.0_f32),
            roll: 0.0_f32,
            pitch: pitch_angle,
            move_forward: 0,
            move_left: 0,
            window: window,
            window_size: na::Vec2::new(100, 100),
            window_center: na::Pnt2::new(50, 50),
        }
    }

    pub fn view(&self) -> na::Mat4<f32> {
        let mut view = self.rotate.to_homogeneous();
        view.set_col(3, self.position);
        match view.inv() {
            Some(x) => x,
            None => panic!("Can't invert view matrix")
        }
    }

    pub fn proj(&self) -> na::Persp3<f32> {
        self.proj
    }

    pub fn update(&mut self, window_size: na::Vec2<u32>) {
        if window_size != self.window_size {
            self.window_size = window_size;
            self.window_center = na::Pnt2::new(window_size.x as i32 / 2, window_size.y as i32 / 2);
            self.proj.set_aspect(window_size.x as f32 / window_size.y as f32 );
            self.window.set_cursor_position(self.window_center.x, self.window_center.y).unwrap();
            // self.window.set_cursor_state(glium::glutin::CursorState::Hide);
        }

        self.rotate = na::Rot3::new_with_euler_angles(self.roll, self.pitch, 0.0_f32);
        if self.move_forward != 0 || self.move_left != 0 {
            let base_dir = na::Vec3::new(self.move_left as f32, 0.0_f32, self.move_forward as f32);
            let dir = (self.rotate * base_dir).normalize() * 0.1_f32;
            self.position.x += dir.x;
            self.position.y += dir.y;
            self.position.z += dir.z;
        }
    }

    pub fn process_input(&mut self, event: &Event) {
        match event {
            &Event::MouseMoved((x, y)) => {
                let dt = self.window_center - na::Pnt2::new(x, y);
                self.window.set_cursor_position(self.window_center.x, self.window_center.y).unwrap();

                let scale = 0.002_f32;
                self.pitch += dt.x as f32 * scale;
                self.roll -= dt.y as f32 * scale;

                const MAX_ROLL: f32 = f32::consts::FRAC_PI_2 - 0.1_f32;
                if self.roll > MAX_ROLL {
                    self.roll = MAX_ROLL;
                }
                if self.roll < -MAX_ROLL {
                    self.roll = -MAX_ROLL;
                }
            }
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.move_forward = 1;
            }
            &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                if self.move_forward == 1 {
                    self.move_forward = 0;
                }
            }
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.move_forward = -1;
            }
            &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                if self.move_forward == -1 {
                    self.move_forward = 0;
                }
            }
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.move_left = -1;
            }
            &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                if self.move_left == -1 {
                    self.move_left = 0;
                }
            }
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.move_left = 1;
            }
            &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                if self.move_left == 1 {
                    self.move_left = 0;
                }
            }
            _ => {}
        }
    }
}
