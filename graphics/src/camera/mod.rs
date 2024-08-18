use geometry::CameraGeometry;

use crate::key::KeyStateMap;

mod geometry;
mod keymap;
pub struct Camera {
    geom: CameraGeometry,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            geom: CameraGeometry::default(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed: 50.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Camera {
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn process_events(&mut self, key_states: &KeyStateMap) {
        key_states
            .iter()
            .filter(|(_, state)| state.is_pressing())
            .for_each(|(key, _)| match key.as_str() {
                keymap::KEY_MOVE_FORWARD => self.geom.move_forward(self.speed),
                keymap::KEY_MOVE_BACKWARD => self.geom.move_backward(self.speed),
                keymap::KEY_MOVE_UP => self.geom.move_up(self.speed),
                keymap::KEY_MOVE_DOWN => self.geom.move_down(self.speed),
                keymap::KEY_MOVE_RIGHT => self.geom.move_right(self.speed),
                keymap::KEY_MOVE_LEFT => self.geom.move_left(self.speed),
                keymap::KEY_ROTATE_RIGHT => self.geom.rotate_right(self.speed),
                keymap::KEY_ROTATE_LEFT => self.geom.rotate_left(self.speed),
                keymap::KEY_ROTATE_UP => self.geom.rotate_up(self.speed),
                keymap::KEY_ROTATE_DOWN => self.geom.rotate_down(self.speed),
                _ => {}
            });
    }

    pub fn build_uniform(&self) -> CameraUniform {
        let view = self.geom.build_view_matrix();
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        CameraUniform {
            view_proj: view_proj.into(),
        }
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
