use crate::key::KeyStateMap;

use super::{geometry::CameraGeometry, CameraUniform};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

const KEY_MOVE_UP: &str = "w";
const KEY_MOVE_DOWN: &str = "s";
const KEY_MOVE_FORWARD: &str = "e";
const KEY_MOVE_BACKWARD: &str = "q";
const KEY_MOVE_RIGHT: &str = "d";
const KEY_MOVE_LEFT: &str = "a";

const KEY_ROTATE_UP: &str = "i";
const KEY_ROTATE_DOWN: &str = "k";
const KEY_ROTATE_RIGHT: &str = "l";
const KEY_ROTATE_LEFT: &str = "j";

pub struct CameraPerspective {
    geom: CameraGeometry,
    speed: f32,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for CameraPerspective {
    fn default() -> Self {
        Self {
            geom: CameraGeometry::default(),
            speed: 50.0,
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

impl CameraPerspective {
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn build_uniform(&self) -> CameraUniform {
        let view = self.geom.build_view_matrix();
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        CameraUniform {
            view_proj: view_proj.into(),
        }
    }

    pub fn process_events(&mut self, key_states: &KeyStateMap) {
        key_states
            .iter()
            .filter(|(_, state)| state.is_pressing())
            .for_each(|(key, _)| match key.as_str() {
                KEY_MOVE_FORWARD => self.geom.move_forward(self.speed),
                KEY_MOVE_BACKWARD => self.geom.move_backward(self.speed),
                KEY_MOVE_UP => self.geom.move_up(self.speed),
                KEY_MOVE_DOWN => self.geom.move_down(self.speed),
                KEY_MOVE_RIGHT => self.geom.move_right(self.speed),
                KEY_MOVE_LEFT => self.geom.move_left(self.speed),
                KEY_ROTATE_RIGHT => self.geom.rotate_right(self.speed),
                KEY_ROTATE_LEFT => self.geom.rotate_left(self.speed),
                KEY_ROTATE_UP => self.geom.rotate_up(self.speed),
                KEY_ROTATE_DOWN => self.geom.rotate_down(self.speed),
                _ => {}
            });
    }
}
