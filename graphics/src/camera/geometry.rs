use cgmath::InnerSpace;

pub struct CameraGeometry {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
}

impl Default for CameraGeometry {
    fn default() -> Self {
        Self {
            eye: (0.0, 0.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
        }
    }
}

impl CameraGeometry {
    pub fn build_view_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up)
    }

    // forward, forward_norm, right, right_norm
    fn axis(
        &self,
    ) -> (
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
    ) {
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let right = forward_norm.cross(cgmath::Vector3::unit_y());
        let right_norm = right.normalize();
        (forward, forward_norm, right, right_norm)
    }

    pub fn move_forward(&mut self, speed: f32) {
        let (_, forward_norm, _, _) = self.axis();
        self.eye += forward_norm * speed;
        self.target += forward_norm * speed;
    }

    pub fn move_backward(&mut self, speed: f32) {
        let (_, forward_norm, _, _) = self.axis();
        self.eye -= forward_norm * speed;
        self.target -= forward_norm * speed;
    }

    pub fn move_up(&mut self, speed: f32) {
        self.eye += cgmath::Vector3::unit_y() * speed;
        self.target += cgmath::Vector3::unit_y() * speed;
    }

    pub fn move_down(&mut self, speed: f32) {
        self.eye -= cgmath::Vector3::unit_y() * speed;
        self.target -= cgmath::Vector3::unit_y() * speed;
    }

    pub fn move_right(&mut self, speed: f32) {
        let (_, _, _, right_norm) = self.axis();
        self.eye += right_norm * speed;
        self.target += right_norm * speed;
    }

    pub fn move_left(&mut self, speed: f32) {
        let (_, _, _, right_norm) = self.axis();
        self.eye -= right_norm * speed;
        self.target -= right_norm * speed;
    }

    pub fn rotate_right(&mut self, speed: f32) {
        let (forward, _, right, _) = self.axis();
        self.target = self.eye + (forward + right * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_left(&mut self, speed: f32) {
        let (forward, _, right, _) = self.axis();
        self.target = self.eye + (forward - right * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_up(&mut self, speed: f32) {
        let (forward, _, _, _) = self.axis();
        self.target = self.eye + (forward + self.up * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_down(&mut self, speed: f32) {
        let (forward, _, _, _) = self.axis();
        self.target = self.eye + (forward - self.up * speed).normalize() * forward.magnitude();
    }
}
