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
    geom_current: CameraGeometry,
    geom_goal: CameraGeometry,
    speed: f32,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl CameraPerspective {
    pub fn new(
        geom: CameraGeometry,
        speed: f32,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            geom_current: geom,
            geom_goal: geom,
            speed,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn build_uniform(&self) -> CameraUniform {
        let view = self.geom_current.build_view_matrix();
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        CameraUniform {
            view_pos: self.geom_current.build_pos_vec().into(),
            target_pos: self.geom_current.build_target_vec().into(),
            view_proj: view_proj.into(),
            aspect: self.aspect,
            _padding: [0.0; 7],
        }
    }

    pub fn process_events(&mut self, key_states: &KeyStateMap) {
        key_states
            .iter()
            .filter(|(_, state)| state.is_pressing())
            .for_each(|(key, _)| match key.as_str() {
                KEY_MOVE_FORWARD => self.geom_goal.move_forward(self.speed),
                KEY_MOVE_BACKWARD => self.geom_goal.move_backward(self.speed),
                KEY_MOVE_UP => self.geom_goal.move_up(self.speed),
                KEY_MOVE_DOWN => self.geom_goal.move_down(self.speed),
                KEY_MOVE_RIGHT => self.geom_goal.move_right(self.speed),
                KEY_MOVE_LEFT => self.geom_goal.move_left(self.speed),
                KEY_ROTATE_RIGHT => self.geom_goal.rotate_right(self.speed),
                KEY_ROTATE_LEFT => self.geom_goal.rotate_left(self.speed),
                KEY_ROTATE_UP => self.geom_goal.rotate_up(self.speed),
                KEY_ROTATE_DOWN => self.geom_goal.rotate_down(self.speed),
                _ => {}
            });
    }

    pub fn tween(&mut self, prop: f32) {
        self.geom_current.tween(&self.geom_goal, prop);
    }
}
