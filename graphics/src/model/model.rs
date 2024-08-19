// code from: https://github.com/sotrh/learn-wgpu

use std::{collections::HashMap, io::BufReader};

use wgpu::{util::DeviceExt, BindGroupDescriptor, BindGroupEntry, BindingResource};

use super::{texture, vertex::ModelVertex};

pub struct Model {
    pub mesh: Mesh,
    pub material: Material,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::TextureSet,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

impl Model {
    pub fn create(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // todo: change this to a remote file
        obj_src: &[u8],
        mtl_src: &[u8], // this will be removed
        texture_diffuse_src: &[u8],
        texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Self> {
        let mut bufreader = BufReader::new(obj_src);

        // this is a temporary solution to load the mtl file
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(&raw_mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mesh = Mesh {
            name: "mesh".to_string(),
            vertex_buffer,
            index_buffer,
            num_elements: raw_mesh.indices.len() as u32,
        };

        let texture =
            texture::TextureSet::from_bytes(device, queue, texture_diffuse_src, "texture")?;

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

        let material = Material {
            name: "Material".to_string(),
            diffuse_texture: texture,
            bind_group: texture_bind_group,
        };

        Ok(Self { mesh, material })
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup) {
        self.draw_mesh(&model.mesh, &model.material, camera_bind_group)
    }
}
