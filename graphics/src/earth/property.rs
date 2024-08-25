use cgmath::InnerSpace;

#[derive(Debug, Clone)]
pub struct EarthProperty {
    pub radius: f32,
    pub rotation: f32,
    pub axis: cgmath::Vector3<f32>,
    pub atmosphere_radius: f32,
}

impl Default for EarthProperty {
    fn default() -> Self {
        let radius = 500.0;
        EarthProperty {
            radius,
            rotation: 0.0,
            axis: cgmath::Vector3::new(0.0, 0.9, 0.15).normalize(),
            atmosphere_radius: radius * 1.025,
        }
    }
}

impl EarthProperty {
    pub fn get_distance_between_earth_and_sun(&self) -> f32 {
        11728.0 * self.radius * 2.0
    }

    pub fn rotate(&mut self, d: f32) {
        self.rotation += d;
    }

    pub fn build_uniform(&self) -> EarthUniform {
        EarthUniform {
            radius: self.radius,
            atmosphere_radius: self.atmosphere_radius,
            axis: self.axis.into(),
            rotation: self.rotation,
            _padding0: 0.0,
            _padding1: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EarthUniform {
    pub radius: f32,
    pub atmosphere_radius: f32,
    pub rotation: f32,
    pub _padding0: f32,
    pub axis: [f32; 3],
    pub _padding1: f32,
}
