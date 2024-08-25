// code from: https://github.com/sotrh/learn-wgpu

use std::io::BufReader;

use super::{texture, vertex::ModelVertex};
use wgpu::{util::DeviceExt, BindGroupDescriptor, BindGroupEntry, BindingResource};

pub struct EarthModel {
    pub mesh: EarthMesh,
    pub texture_bind_group: wgpu::BindGroup,
}

pub struct AtmosphereModel {
    pub mesh: EarthMesh,
}

pub struct EarthMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

pub async fn create_earth_and_atmosphere_model(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    obj_src: &[u8],
    mtl_src: &[u8],
    texture_diffuse_src: &[u8],
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<(EarthModel, AtmosphereModel)> {
    let mut bufreader = BufReader::new(obj_src);

    let raw_mtl = tobj::load_mtl_buf(&mut BufReader::new(mtl_src))?;

    let (raw_models, _) = tobj::load_obj_buf(
        &mut bufreader,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |_| tobj::MTLLoadResult::Ok(raw_mtl.clone()),
    )?;

    let raw_model = &raw_models.get(0).expect("No model loaded");
    let raw_mesh = &raw_model.mesh;

    let mut vertices = Vec::new();
    for i in 0..raw_mesh.positions.len() / 3 {
        vertices.push(ModelVertex {
            position: [
                raw_mesh.positions[i * 3],
                raw_mesh.positions[i * 3 + 1],
                raw_mesh.positions[i * 3 + 2],
            ],
            tex_coords: [
                raw_mesh.texcoords[i * 2],
                1.0 - raw_mesh.texcoords[i * 2 + 1],
            ],
            normal: [
                raw_mesh.normals[i * 3],
                raw_mesh.normals[i * 3 + 1],
                raw_mesh.normals[i * 3 + 2],
            ],
        });
    }
    let (earth_mesh, atmosphere_mesh) = {
        let mut meshes_iter = (0..2).map(|i| {
            let label = match i {
                0 => "earth",
                1 => "atmosphere",
                _ => unreachable!(),
            };

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(label.to_string() + " vertex buffer")),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(label.to_string() + " index buffer")),
                contents: bytemuck::cast_slice(&raw_mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            EarthMesh {
                vertex_buffer,
                index_buffer,
                num_elements: raw_mesh.indices.len() as u32,
            }
        });
        (meshes_iter.next().unwrap(), meshes_iter.next().unwrap())
    };

    let texture = texture::TextureSet::from_bytes(device, queue, texture_diffuse_src, "texture")?;

    let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("material_bind_group"),
        layout: texture_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&texture.view()),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&texture.sampler()),
            },
        ],
    });

    Ok((
        EarthModel {
            mesh: earth_mesh,
            texture_bind_group,
        },
        AtmosphereModel {
            mesh: atmosphere_mesh,
        },
    ))
}

pub trait DrawModel<'a> {
    fn draw_earth_model(
        &mut self,
        model: &'a EarthModel,
        camera_bind_group: &'a wgpu::BindGroup,
        sun_bind_group: &'a wgpu::BindGroup,
    );

    fn draw_atmosphere_model(
        &mut self,
        model: &'a AtmosphereModel,
        camera_bind_group: &'a wgpu::BindGroup,
        sun_bind_group: &'a wgpu::BindGroup,
        earth_property_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn draw_earth_model(
        &mut self,
        model: &'a EarthModel,
        camera_bind_group: &'a wgpu::BindGroup,
        sun_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, model.mesh.vertex_buffer.slice(..));
        self.set_index_buffer(model.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, &model.texture_bind_group, &[]);
        self.set_bind_group(2, &sun_bind_group, &[]);
        self.draw_indexed(0..model.mesh.num_elements, 0, 0..1);
    }

    fn draw_atmosphere_model(
        &mut self,
        model: &'a AtmosphereModel,
        camera_bind_group: &'a wgpu::BindGroup,
        sun_bind_group: &'a wgpu::BindGroup,
        earth_property_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, model.mesh.vertex_buffer.slice(..));
        self.set_index_buffer(model.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, sun_bind_group, &[]);
        self.set_bind_group(2, earth_property_bind_group, &[]);
        self.draw_indexed(0..model.mesh.num_elements, 0, 0..1);
    }
}
