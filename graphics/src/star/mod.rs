use cgmath::InnerSpace;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StarInstanceRaw {
    direction: [f32; 3],
    distance: f32,
}

impl StarInstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<StarInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

pub struct Star {
    pub instances: Vec<StarInstanceRaw>,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Star {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let instances = (0..1000)
            .map(|_| {
                let x = rand::random::<f32>() * 2.0 - 1.0;
                let y = rand::random::<f32>() * 2.0 - 1.0;
                let z = rand::random::<f32>() * 2.0 - 1.0;
                let dir = cgmath::Vector3::new(x, y, z).normalize();
                StarInstanceRaw {
                    direction: dir.into(),
                    distance: 3.0 - rand::random::<f32>().powf(2.0),
                }
            })
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: (std::mem::size_of::<StarInstanceRaw>() * instances.len()) as u64,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("star_index_buffer"),
            contents: bytemuck::cast_slice(&[0u16, 1, 2]),
            usage: wgpu::BufferUsages::INDEX,
        });

        queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(&instances));

        Self {
            instances,
            index_buffer,
            instance_buffer,
            num_indices: 3,
        }
    }
}
