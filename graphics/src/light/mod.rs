use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferUsages, ShaderStages,
};

pub struct Light {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    position: cgmath::Point3<f32>,
    color: cgmath::Point3<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
struct LightUniform {
    position: [f32; 3],
    _padding0: u32,
    color: [f32; 3],
    _padding1: u32,
}

impl Light {
    pub fn new(
        device: &wgpu::Device,
        position: cgmath::Point3<f32>,
        color: cgmath::Point3<f32>,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("light_bind_group_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("light"),
            contents: bytemuck::cast_slice(&[Self::build_uniform(position, color)]),
            usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("light_bind_group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
            position,
            color,
        }
    }

    fn build_uniform(position: cgmath::Point3<f32>, color: cgmath::Point3<f32>) -> LightUniform {
        return LightUniform {
            position: position.into(),
            color: color.into(),
            ..LightUniform::default()
        };
    }
}
