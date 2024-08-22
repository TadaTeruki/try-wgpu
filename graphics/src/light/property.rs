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

pub type LightVertex = LightUniform;

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

    pub fn build_vertex(&self) -> LightVertex {
        return LightVertex {
            position: self.position.into(),
            color: self.color.into(),
            ..LightVertex::default()
        };
    }
}

impl LightVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<LightVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 1,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: 2,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 3,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}
