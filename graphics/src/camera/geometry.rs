use cgmath::InnerSpace;

#[derive(Debug, Clone, Copy)]
pub struct CameraGeometry {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up_axis: cgmath::Vector3<f32>,
}

impl CameraGeometry {
    pub fn new(
        eye: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up_axis: cgmath::Vector3<f32>,
    ) -> Self {
        Self {
            eye,
            target,
            up_axis,
        }
    }

    pub fn tween(&mut self, goal: &CameraGeometry, prop: f32) {
        self.eye = cgmath::Point3::new(
            self.eye.x + (goal.eye.x - self.eye.x) * prop,
            self.eye.y + (goal.eye.y - self.eye.y) * prop,
            self.eye.z + (goal.eye.z - self.eye.z) * prop,
        );
        self.target = cgmath::Point3::new(
            self.target.x + (goal.target.x - self.target.x) * prop,
            self.target.y + (goal.target.y - self.target.y) * prop,
            self.target.z + (goal.target.z - self.target.z) * prop,
        );
        self.up_axis = cgmath::Vector3::new(
            self.up_axis.x + (goal.up_axis.x - self.up_axis.x) * prop,
            self.up_axis.y + (goal.up_axis.y - self.up_axis.y) * prop,
            self.up_axis.z + (goal.up_axis.z - self.up_axis.z) * prop,
        );
    }

    pub fn build_view_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up_axis)
    }

    pub fn build_pos_vec(&self) -> cgmath::Vector4<f32> {
        self.eye.to_homogeneous()
    }

    pub fn build_target_vec(&self) -> cgmath::Vector4<f32> {
        self.target.to_homogeneous()
    }

    // forward, forward_norm, right, right_norm, up, up_norm
    fn axis(
        &self,
    ) -> (
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
        cgmath::Vector3<f32>,
    ) {
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let right = forward_norm.cross(self.up_axis);
        let right_norm = right.normalize();
        let up = right_norm.cross(forward_norm);
        let up_norm = up.normalize();
        (forward, forward_norm, right, right_norm, up, up_norm)
    }

    pub fn move_forward(&mut self, speed: f32) {
        let (_, forward_norm, _, _, _, _) = self.axis();
        self.eye += forward_norm * speed;
        self.target += forward_norm * speed;
    }

    pub fn move_backward(&mut self, speed: f32) {
        let (_, forward_norm, _, _, _, _) = self.axis();
        self.eye -= forward_norm * speed;
        self.target -= forward_norm * speed;
    }

    pub fn move_up(&mut self, speed: f32) {
        let (_, _, _, _, _, up_norm) = self.axis();
        self.eye += up_norm * speed;
        self.target += up_norm * speed;
    }

    pub fn move_down(&mut self, speed: f32) {
        let (_, _, _, _, _, up_norm) = self.axis();
        self.eye -= up_norm * speed;
        self.target -= up_norm * speed;
    }

    pub fn move_right(&mut self, speed: f32) {
        let (_, _, _, right_norm, _, _) = self.axis();
        self.eye += right_norm * speed;
        self.target += right_norm * speed;
    }

    pub fn move_left(&mut self, speed: f32) {
        let (_, _, _, right_norm, _, _) = self.axis();
        self.eye -= right_norm * speed;
        self.target -= right_norm * speed;
    }

    pub fn rotate_right(&mut self, speed: f32) {
        let (forward, _, right, _, _, _) = self.axis();
        self.target = self.eye + (forward + right * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_left(&mut self, speed: f32) {
        let (forward, _, right, _, _, _) = self.axis();
        self.target = self.eye + (forward - right * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_up(&mut self, speed: f32) {
        let (forward, _, _, _, up, _) = self.axis();
        self.target = self.eye + (forward + up * speed).normalize() * forward.magnitude();
    }

    pub fn rotate_down(&mut self, speed: f32) {
        let (forward, _, _, _, up, _) = self.axis();
        self.target = self.eye + (forward - up * speed).normalize() * forward.magnitude();
    }
}
