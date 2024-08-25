pub mod model;
pub mod property;
mod texture;
pub mod vertex;

use model::EarthModel;
use property::EarthProperty;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferUsages, ShaderStages,
};

pub struct Earth {
    pub property: EarthProperty,
    pub model: EarthModel,
    pub uniform_buffer: Buffer,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl Earth {
    pub fn new(device: &wgpu::Device, model: EarthModel, property: EarthProperty) -> Self {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("earth_property_bind_group_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("earth_property"),
            contents: bytemuck::cast_slice(&[property.build_uniform()]),
            usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("earth_property_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            property,
            model,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
        }
    }

    pub fn enque_update_uniform(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.property.build_uniform()]),
        );
    }
}
