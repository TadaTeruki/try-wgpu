use cgmath::InnerSpace;

use crate::key::{KeyState, KeyStateMap};

use super::Camera;

pub struct CameraController {
    speed: f32,
    move_forward_pressed: bool,
    move_backward_pressed: bool,
    move_left_pressed: bool,
    move_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            move_forward_pressed: false,
            move_backward_pressed: false,
            move_left_pressed: false,
            move_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, key_states: KeyStateMap) {
        key_states
            .iter()
            .for_each(|(key, state)| match key.as_str() {
                "w" | "ArrowUp" => {
                    self.move_forward_pressed = *state != KeyState::Release;
                }
                "s" | "ArrowDown" => {
                    self.move_backward_pressed = *state != KeyState::Release;
                }
                "a" | "ArrowLeft" => {
                    self.move_left_pressed = *state != KeyState::Release;
                }
                "d" | "ArrowRight" => {
                    self.move_right_pressed = *state != KeyState::Release;
                }
                _ => {}
            });
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.move_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.move_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.move_right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.move_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
