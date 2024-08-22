use wgpu::{util::DeviceExt, RenderPipelineDescriptor};

use crate::camera::Camera;

const SKY_VERTICES: &[f32] = &[
    -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0,
];

const SKY_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
struct Sky {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Sky {
    pub fn new(device: &wgpu::Device, camera: &Camera, config: &wgpu::SurfaceConfiguration) -> Self {


        let vertext_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sky_vertex_buffer"),
            contents: bytemuck::cast_slice(&SKY_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sky_index_buffer"),
            contents: bytemuck::cast_slice(&SKY_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
    
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader/sky.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sky_render_pipeline_layout"),
                bind_group_layouts: &[&camera.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("sky_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        return Self {
            pipeline: render_pipeline,
            vertex_buffer: vertext_buffer,
            index_buffer: index_buffer,
            num_indices: SKY_INDICES.len() as u32,
        };

    }
}
