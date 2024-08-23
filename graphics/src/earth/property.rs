use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferUsages, ShaderStages,
};

pub struct EarthPropertyBinding {
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl EarthPropertyBinding {
    pub fn new(device: &wgpu::Device, property: EarthProperty) -> Self {
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
            contents: bytemuck::cast_slice(&[property]),
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
            uniform_bind_group,
            uniform_bind_group_layout,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EarthProperty {
    pub radius: f32,
    pub atmosphere_radius: f32,
}

impl Default for EarthProperty {
    fn default() -> Self {
        let radius = 500.0;
        EarthProperty {
            radius,
            atmosphere_radius: radius * 1.025,
        }
    }
}

impl EarthProperty {
    pub fn get_distance_between_earth_and_sun(&self) -> f32 {
        11728.0 * self.radius * 2.0
    }
}
