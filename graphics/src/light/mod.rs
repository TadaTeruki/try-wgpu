use property::LightProperty;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferUsages, ShaderStages,
};

pub mod property;

pub struct Light {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub vertex_buffer: wgpu::Buffer,
    property: LightProperty,
}

impl Light {
    pub fn new(device: &wgpu::Device, property: LightProperty) -> Self {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("light"),
            contents: bytemuck::cast_slice(&[property.build_uniform()]),
            usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("light_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("light_vertex_buffer"),
            contents: bytemuck::cast_slice(&[property.build_vertex()]),
            usage: BufferUsages::VERTEX,
        });

        Self {
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
            vertex_buffer,
            property,
        }
    }
}
