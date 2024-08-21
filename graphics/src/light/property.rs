pub struct LightProperty {
    position: cgmath::Point3<f32>,
    color: cgmath::Point3<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct LightUniform {
    position: [f32; 3],
    _padding0: u32,
    color: [f32; 3],
    _padding1: u32,
}

impl LightProperty {
    pub fn new(position: cgmath::Point3<f32>, color: cgmath::Point3<f32>) -> Self {
        Self { position, color }
    }

    pub fn build_uniform(&self) -> LightUniform {
        return LightUniform {
            position: self.position.into(),
            color: self.color.into(),
            ..LightUniform::default()
        };
    }
}
